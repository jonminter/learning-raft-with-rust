pub(crate) mod common;
pub(crate) mod invariant_checker;
pub(crate) mod sim_log;
pub(crate) mod sim_network;
pub(crate) mod sim_process;
pub(crate) mod sim_transport;

use fault_injection::{set_trigger_function, FAULT_INJECT_COUNTER};
use mock_instant::MockClock;
use raft_consensus::{RaftConfig, ServerId};
use tracing::{debug, warn};
use tracing::{info, trace};

use invariant_checker::InvariantChecker;

use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;
use std::sync::mpsc;
use std::time::Duration;

use rand_chacha::ChaCha8Rng;

use crate::simulator::common::SimulatorAction;
use crate::simulator::sim_log::SimLogEntry;

use self::common::SimTime;
use self::common::SimulatorEvent;
use self::common::WakeUpAtOrBefore;
use self::invariant_checker::ServerProcessRaftStateEventCollector;
use self::sim_log::SimLog;
use self::sim_network::SimNetwork;
use self::sim_process::SimRaftProcess;

fn io_fault_injection_trigger_fn(crate_name: &str, file_name: &str, line_number: u32) {
    println!(
        "fault injected at {} {} {}",
        crate_name, file_name, line_number
    );
    FAULT_INJECT_COUNTER.store(
        FAIL_EVERY_N_IO_OPS.load(std::sync::atomic::Ordering::Acquire),
        std::sync::atomic::Ordering::Release,
    );
}

const FAIL_EVERY_N_IO_OPS: AtomicU64 = AtomicU64::new(u64::MAX);

/// A simulation of a cluster of Raft servers.
/// This is used to test the Raft algorithm in a controlled environment.
/// The simulation is deterministic and can be run multiple times with the same inputs as long as you use a random number generator with the same seed.
/// The simulation is also fast, as it does not use real time.
pub(crate) struct ClusterSim {
    rng: ChaCha8Rng,
    servers: HashMap<ServerId, SimRaftProcess<ServerProcessRaftStateEventCollector>>,
    network: SimNetwork,
    transport_wake_up_rx: mpsc::Receiver<WakeUpAtOrBefore>,
    events_to_process: BinaryHeap<Reverse<SimulatorEvent>>,
    invariant_checker: InvariantChecker,
    pub(crate) results: SimResults,
    pub(crate) log: SimLog,
    transport_wakeup_requests: BTreeSet<SimTime>,
}

pub(crate) struct SimResults {
    pub(crate) was_leader_elected: bool,
    pub(crate) all_elected_leaders: HashSet<ServerId>,
}

impl ClusterSim {
    pub(crate) fn new(
        num_servers: u64,
        mut network: SimNetwork,
        config: RaftConfig,
        rng: ChaCha8Rng,
        storage_temp_dir: String,
        log_file_path: Option<PathBuf>,
    ) -> Self {
        assert_eq!(
            num_servers,
            network.server_ids.len() as u64,
            "Network should have the same number of servers as the cluster"
        );
        set_trigger_function(io_fault_injection_trigger_fn);
        MockClock::set_time(Duration::from_millis(0));

        let timer_rx = network.take_timer_rx();

        let log = SimLog::new(log_file_path);

        let invariant_checker = InvariantChecker::new();

        let mut servers = HashMap::new();
        let mut server_ids = HashSet::new();
        for s in 0..num_servers {
            let sid = ServerId(s);
            // Server ID should have connection in network
            assert!(
                network.server_ids.contains(&sid),
                "Server {server:?} should have a connection in the network",
                server = s
            );
            server_ids.insert(sid);
        }
        for s in 0..num_servers {
            let sid = ServerId(s);
            let process = SimRaftProcess::new(
                sid,
                num_servers,
                config.clone(),
                storage_temp_dir.clone(),
                rng.clone(),
                &mut network,
                invariant_checker.event_collector_for_server(),
            );
            servers.insert(sid, process);
        }

        let messages = BinaryHeap::new();

        ClusterSim {
            servers,
            network,
            events_to_process: messages,
            rng,
            transport_wake_up_rx: timer_rx,
            results: SimResults {
                was_leader_elected: false,
                all_elected_leaders: HashSet::new(),
            },
            log,
            invariant_checker,
            transport_wakeup_requests: BTreeSet::new(),
        }
    }

    pub(crate) fn reset_results(&mut self) {
        self.results.was_leader_elected = false;
        self.results.all_elected_leaders = HashSet::new();
        self.log.reset();
    }

    /// Provides a way for tests to inject messages into the simulation.
    pub(crate) fn enqueue_event(&mut self, msg: SimulatorEvent) {
        assert!(
            msg.time >= SimTime::now(),
            "Cannot enqueue an event in the past {msg:?} (sim time = {sim_time:?}!",
            msg = msg,
            sim_time = SimTime::now()
        );

        self.log
            .push(SimLogEntry::event_queued(SimTime::now(), &msg));
        self.events_to_process.push(Reverse(msg));
    }

    /// Runs a single step of the simulation, this doess...
    /// 1. Interrupt network transports for each node so they can check the current simulation time and time out waiting if necessary
    /// 2. Fetch all messages that have been sent over the nextwork since the last iteration
    /// 3. Get the next simulator message to be processed and run the appropriate action (i.e. deliver a message to a server, partion the network, etc.)
    /// 4. Checks that no Raft invariants have been violated in the cluster
    fn run_step(&mut self) {
        for (_, server_process) in self.servers.iter_mut() {
            server_process.restart_if_needed(&mut self.network);
        }

        let outbound_messages = self
            .network
            .get_all_queued_outbound_messages(&mut self.rng, &mut self.log);
        for (message, delivery_time) in outbound_messages {
            self.enqueue_event(SimulatorEvent {
                time: delivery_time,
                action: SimulatorAction::SendOverNetwork(message),
            });
        }

        let maybe_next = self.events_to_process.peek();
        // Find the first transport wakeup request that is before the next event to process (if there is one)
        let transport_wake_up_requests: HashSet<WakeUpAtOrBefore> =
            self.transport_wake_up_rx.try_iter().collect();
        for wake_up_by in transport_wake_up_requests {
            self.transport_wakeup_requests
                .insert(if wake_up_by.0 >= SimTime::now() {
                    wake_up_by.0
                } else {
                    SimTime::now()
                });
        }

        let maybe_wakeup_time = self
            .transport_wakeup_requests
            .iter()
            .filter(|wake_up| maybe_next.is_none() || wake_up <= &&maybe_next.unwrap().0.time)
            .next()
            .cloned();

        if let Some(wakeup_time) = maybe_wakeup_time {
            let advance_by = wakeup_time.checked_sub(&SimTime::now()).expect(
                format!(
                    "Time should not go backwards, wake up time {wakeup_time:?} is in the past (sim time = {sim_time:?}!",
                    wakeup_time = wakeup_time,
                    sim_time=SimTime::now()
                )
                .as_str(),
            );
            MockClock::advance(advance_by);
            for (_, server_process) in self.servers.iter_mut() {
                server_process.wake_up_transport_connector();
            }
            self.transport_wakeup_requests.remove(&wakeup_time);
        } else if !self.events_to_process.is_empty() {
            let next = self.events_to_process.pop().unwrap().0;
            self.log
                .push(SimLogEntry::event_processed(SimTime::now(), &next));

            let advance_duration = next.time.checked_sub(&SimTime::now());
            if let Some(advance_duration) = advance_duration {
                MockClock::advance(advance_duration);
            }

            trace!(
                "Performing action: {:?} at time {:?}ms",
                next.action,
                next.time.0.as_millis()
            );

            match next.action {
                SimulatorAction::SendOverNetwork(network_message) => {
                    trace!(
                            "DELIVER NETWORK MESSAGE: msg_time = {time:?}ms, mock_time={mock_time:?}ms -- ({from:?} -> {to:?}): {rpc_message:?}",
                            time = next.time.as_millis(),
                            mock_time = SimTime::now().as_millis(),
                            rpc_message = network_message,
                            from = network_message.from(),
                            to = network_message.to(),
                        );

                    self.network
                        .deliver_message(network_message.to(), network_message);
                }
                SimulatorAction::PartitionNetwork(partitions) => {
                    trace!(
                            "PARTITION NETWORK: msg_time = {time:?}ms, mock_time={mock_time:?}ms -- Partitioning network: {partition:?}",
                            time = next.time.as_millis(),
                            mock_time = SimTime::now().as_millis(),
                            partition = partitions,
                        );
                    self.network.partition_network(partitions);
                }
                SimulatorAction::HealNetworkPartition => self.network.heal_network_partition(),
                SimulatorAction::InjectIOFailureEveryNOps(n) => {
                    FAIL_EVERY_N_IO_OPS.store(n, std::sync::atomic::Ordering::Release);
                    FAULT_INJECT_COUNTER.store(1, std::sync::atomic::Ordering::Release);
                }
                SimulatorAction::RestoreIOFunctioning => {
                    FAULT_INJECT_COUNTER.store(u64::MAX, std::sync::atomic::Ordering::Release);
                }
            }

            self.invariant_checker
                .check_invariants(SimTime::now(), &mut self.log);

            self.invariant_checker.get_current_leader().map(|leader| {
                self.results.was_leader_elected = true;
                self.results.all_elected_leaders.insert(leader);
            });
        }
    }

    /// Runs the simulation until the given time has been reached.
    pub(crate) fn run_until_time(&mut self, time: Duration) {
        info!(
            "Running simulation: current time = {current_time:?}, run until = {run_until:?}",
            current_time = MockClock::time(),
            run_until = time
        );
        let mut last_time_log = Duration::from_millis(0);
        while MockClock::time() <= time {
            if MockClock::time() - last_time_log >= Duration::from_millis(1000) {
                info!(
                    "Current simulator time {time:?}ms",
                    time = MockClock::time().as_millis()
                );
                last_time_log = MockClock::time();
            }
            trace!(
                "Simulation time = {time:?}ms",
                time = MockClock::time().as_millis()
            );
            let time_before_step = MockClock::time();
            self.run_step();
            let time_after_step = MockClock::time();

            assert!(
                time_after_step >= time_before_step,
                "Simulator time went backwards! This is a bug in the simulator!"
            )
        }
        info!(
            "Finished simulation! time = {current_time:?}ms",
            current_time = MockClock::time().as_millis()
        );

        if let Err(_) = self.log.flush() {
            panic!("Failed to flush simulation log to disk, it may be incomplete!");
        }
    }
}
