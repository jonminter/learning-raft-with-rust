pub(crate) mod common;
pub(crate) mod invariant_checker;
pub(crate) mod sim_log;
pub(crate) mod sim_network;
pub(crate) mod sim_process;
pub(crate) mod sim_transport;

use mock_instant::MockClock;
use raft_consensus::{RaftConfig, ServerId};
use tracing::{info, trace};

use invariant_checker::InvariantChecker;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use rand_chacha::ChaCha8Rng;

use crate::simulator::common::SimulatorAction;
use crate::simulator::sim_log::SimLogEntry;

use self::common::ClockAdvance;
use self::common::SimTime;
use self::common::SimulatorEvent;
use self::sim_log::SimLog;
use self::sim_network::SimNetwork;
use self::sim_process::SimRaftProcess;

/// A simulation of a cluster of Raft servers.
/// This is used to test the Raft algorithm in a controlled environment.
/// The simulation is deterministic and can be run multiple times with the same inputs as long as you use a random number generator with the same seed.
/// The simulation is also fast, as it does not use real time.
pub(crate) struct ClusterSim {
    rng: ChaCha8Rng,
    servers: HashMap<ServerId, SimRaftProcess>,
    network: SimNetwork,
    tick_requests: mpsc::Receiver<ClockAdvance>,
    events_to_process: BinaryHeap<SimulatorEvent>,
    invariant_checker: InvariantChecker,
    pub(crate) results: SimResults,
    pub(crate) log: SimLog,
    current_time: SimTime,
    last_queued_tick_request: Option<SimTime>,
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
        log_file_path: &Path,
    ) -> Self {
        assert_eq!(
            num_servers,
            network.server_ids.len() as u64,
            "Network should have the same number of servers as the cluster"
        );
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
                network.take_transport_for(sid),
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
            tick_requests: timer_rx,
            results: SimResults {
                was_leader_elected: false,
                all_elected_leaders: HashSet::new(),
            },
            log,
            invariant_checker,
            current_time: SimTime(MockClock::time()),
            last_queued_tick_request: None,
        }
    }

    pub(crate) fn reset_results(&mut self) {
        self.results.was_leader_elected = false;
        self.results.all_elected_leaders = HashSet::new();
        self.log.reset();
    }

    /// Provides a way for tests to inject messages into the simulation.
    pub(crate) fn enqueue_event(&mut self, msg: SimulatorEvent) {
        self.log
            .push(SimLogEntry::event_queued(self.current_time, &msg));
        self.events_to_process.push(msg);
    }

    /// Runs a single step of the simulation, this doess...
    /// 1. Interrupt network transports for each node so they can check the current simulation time and time out waiting if necessary
    /// 2. Fetch all messages that have been sent over the nextwork since the last iteration
    /// 3. Get the next simulator message to be processed and run the appropriate action (i.e. deliver a message to a server, partion the network, etc.)
    /// 4. Checks that no Raft invariants have been violated in the cluster
    fn run_step(&mut self) {
        self.current_time = SimTime(MockClock::time());

        let tick_requests: Vec<ClockAdvance> = self.tick_requests.try_iter().collect();
        for advance_clock_by in tick_requests {
            let tick_at = self.current_time + advance_clock_by;

            if self.last_queued_tick_request.is_none()
                || self.last_queued_tick_request.unwrap() < tick_at
            {
                self.enqueue_event(SimulatorEvent {
                    time: tick_at,
                    action: SimulatorAction::TickClock,
                });
                self.last_queued_tick_request = Some(self.current_time);
            }
        }

        let outbound_messages = self.network.get_all_queued_outbound_messages(&mut self.rng);
        for (message, delivery_time) in outbound_messages {
            self.enqueue_event(SimulatorEvent {
                time: delivery_time,
                action: SimulatorAction::SendOverNetwork(message),
            });
        }

        if !self.events_to_process.is_empty() {
            let next = self.events_to_process.pop().unwrap();
            self.log
                .push(SimLogEntry::event_processed(self.current_time, &next));

            let advance_duration = next.time.checked_sub(&self.current_time);
            if let Some(advance_duration) = advance_duration {
                MockClock::advance(advance_duration.into());
            }
            self.current_time = MockClock::time().into();

            trace!(
                "Performing action: {:?} at time {:?}ms",
                next.action,
                next.time.0.as_millis()
            );

            match next.action {
                SimulatorAction::TickClock => {
                    // Wakes up any servers waiting for messages so they can update their clocks
                    // and allow all servers to perform any time sensitive actions
                    for (_, server_process) in self.servers.iter_mut() {
                        server_process.tick();
                    }
                }

                SimulatorAction::SendOverNetwork(network_message) => {
                    trace!(
                            "DELIVER NETWORK MESSAGE: msg_time = {time:?}ms, mock_time={mock_time:?}ms -- ({from:?} -> {to:?}): {rpc_message:?}",
                            time = next.time.as_millis(),
                            mock_time = self.current_time.as_millis(),
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
                            mock_time = self.current_time.as_millis(),
                            partition = partitions,
                        );
                    self.network.partition_network(partitions);
                }
                SimulatorAction::HealNetworkPartition => self.network.heal_network_partition(),
            }

            self.invariant_checker
                .check_invariants(self.current_time, &mut self.log);

            self.invariant_checker.get_current_leader().map(|leader| {
                self.results.was_leader_elected = true;
                self.results.all_elected_leaders.insert(leader);
            });
        }
    }

    /// Runs the simulation until the given time has been reached.
    pub(crate) fn run_until_time(&mut self, time: Duration) {
        info!(
            "Running simulation: current time = {current_time:?}, run until = {run_until:?}ms",
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
            self.run_step();
        }
        info!(
            "Finished simulation! time = {current_time:?}ms",
            current_time = MockClock::time().as_millis()
        )
    }
}
