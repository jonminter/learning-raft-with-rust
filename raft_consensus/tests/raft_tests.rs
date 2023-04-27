/// Tests consensus with simulator
use crate::simulator::{
    common::{SimTime, SimulatorAction, SimulatorEvent},
    sim_network::{LatencyMean, LatencyStdDev, PacketLossProbability, SimNetwork},
    ClusterSim,
};
use lazy_static::lazy_static;
use quickcheck::{Arbitrary, QuickCheck, Testable};
use raft_consensus::{RaftConfig, ServerId};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::num_traits::ToPrimitive;
use simulator::sim_log::SimLog;
use std::{
    char::MAX,
    collections::{BTreeSet, HashMap, HashSet},
    path::{Path, PathBuf},
    time::Duration,
};
use tempfile::TempDir;
use tracing::{debug, error, info};
mod simulator;
use fault_injection::{set_trigger_function, FAULT_INJECT_COUNTER};
use test_log::test;

// Use quickcheck to implement some stateful tests
// Generate a series of ops
// - PartitionNetwork - Creates a network partition separating a set of nodes from the rest of the cluster
// - HealPartition - Makes the network whole again, reversing the effect of PartitionNetwork
// - RunForDuration - Runs the simulation for a given duration without any other actions
// - FailNode - Put one node in a network partition to simulate a failure of a node
// - RecoverNode - Heal the network partition to simulate a recovery of a node

fn new_rng(maybe_seed: Option<u64>) -> ChaCha8Rng {
    match maybe_seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => {
            let mut rng = ChaCha8Rng::from_entropy();
            let seed = rng.next_u64();
            println!("====================================");
            println!("RNG SEED FOR TESTS: {seed}", seed = seed);
            println!("====================================");
            ChaCha8Rng::seed_from_u64(seed)
        }
    }
}

fn sim_log_path(log_file_name: Option<&str>) -> Option<PathBuf> {
    log_file_name.map(|filename| {
        let pwd = std::env::current_dir().unwrap();
        pwd.join("..").join(filename)
    })
}

const SIMULATION_DURATION: Duration = Duration::from_secs(300);

#[test]
fn should_elect_leader_without_network_partition() {
    let rng = new_rng(None);
    let config = RaftConfig {
        leader_heartbeat_interval: Duration::from_millis(100),
        min_election_timeout_ms: 150,
        max_election_timeout_ms: 300,
    };

    let network = SimNetwork::with_defaults(
        5,
        PacketLossProbability(0.01),
        LatencyMean(5.0),
        LatencyStdDev(2.0),
    );
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap();
    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        sim_log_path(None),
    );

    sim.run_until_time(SIMULATION_DURATION);

    assert_eq!(sim.results.was_leader_elected, true);
}

#[test]
fn should_elect_leader_during_network_partition_if_we_have_quorum() {
    let rng = new_rng(None);
    let config = RaftConfig {
        leader_heartbeat_interval: Duration::from_millis(100),
        min_election_timeout_ms: 150,
        max_election_timeout_ms: 300,
    };

    let network = SimNetwork::with_defaults(
        5,
        PacketLossProbability(0.01),
        LatencyMean(5.0),
        LatencyStdDev(2.0),
    );
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap();

    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        sim_log_path(None),
    );

    info!("Current sim time is {time:?}", time = SimTime::now());

    sim.enqueue_event(SimulatorEvent {
        time: SimTime::now(),
        action: SimulatorAction::PartitionNetwork(vec![
            {
                let mut partition = HashSet::new();
                partition.insert(ServerId(0));
                partition.insert(ServerId(1));
                partition.insert(ServerId(3));
                partition
            },
            {
                let mut partition = HashSet::new();
                partition.insert(ServerId(2));
                partition.insert(ServerId(4));
                partition
            },
        ]),
    });

    sim.run_until_time(SIMULATION_DURATION);
    assert_eq!(sim.results.was_leader_elected, true);

    // 2 & 4 in a partition without quorum, they should not be able to be elected leader
    assert!(!sim.results.all_elected_leaders.contains(&ServerId(2)));
    assert!(!sim.results.all_elected_leaders.contains(&ServerId(4)));
}

#[test]
fn should_not_be_able_to_elect_leader_without_quorum() {
    let rng = new_rng(None);
    let config = RaftConfig {
        leader_heartbeat_interval: Duration::from_millis(100),
        min_election_timeout_ms: 150,
        max_election_timeout_ms: 300,
    };

    let network = SimNetwork::with_defaults(
        5,
        PacketLossProbability(0.01),
        LatencyMean(5.0),
        LatencyStdDev(2.0),
    );
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap();
    let pwd = std::env::current_dir().unwrap();

    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        sim_log_path(None),
    );

    sim.enqueue_event(SimulatorEvent {
        time: SimTime::from_millis(0),
        action: SimulatorAction::PartitionNetwork(vec![
            {
                let mut partition = HashSet::new();
                partition.insert(ServerId(0));
                partition.insert(ServerId(1));
                partition
            },
            {
                let mut partition = HashSet::new();
                partition.insert(ServerId(2));
                partition.insert(ServerId(3));
                partition
            },
            {
                let mut partition = HashSet::new();
                partition.insert(ServerId(4));
                partition
            },
        ]),
    });

    sim.run_until_time(SIMULATION_DURATION);
    assert_eq!(sim.results.was_leader_elected, false);
    drop(sim);
}

#[derive(Debug, Clone)]
struct SimInstructionSequence {
    generated_state_changes: Vec<SimulatorEvent>,
}

impl SimInstructionSequence {}

const NUM_NODES_IN_CLUSTER: usize = 5;
const NODES: [ServerId; 5] = [
    ServerId(0),
    ServerId(1),
    ServerId(2),
    ServerId(3),
    ServerId(4),
];

const CLOCK_ADVANCE_CHOICES: [u64; 9] = [100, 500, 100, 1000, 500, 1000, 100, 5000, 10000];
const MAX_TIME_BETWEEN_INSTRUCTIONS: usize = 60_000;
const INSTRUCTION_PARTITION_NETWORK: &str = "PartitionNetwork";
const INSTRUCTION_HEAL_NETWORK_PARTITION: &str = "HealNetworkPartition";
const INSTRUCTION_FAIL_NODE: &str = "FailNode";
const INSTRUCTION_RECOVER_NODE: &str = "RecoverNode";
const FAIL_NEXT_IO_OPERATION: &str = "FailNextIOOperation";

fn io_fault_injection_trigger_fn(crate_name: &str, file_name: &str, line_number: u32) {
    println!(
        "fault injected at {} {} {}",
        crate_name, file_name, line_number
    );
    FAULT_INJECT_COUNTER.store(u64::MAX, std::sync::atomic::Ordering::Release);
}

impl Arbitrary for SimInstructionSequence {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let failed_nodes = HashSet::<ServerId>::new();
        let mut network_partition: Option<Vec<HashSet<ServerId>>> = None;

        let mut sequence_of_events = Vec::<SimulatorEvent>::new();

        let mut clock: u64 = 0;

        let num_instructions = g.size();
        debug!("num_instructions: {}", num_instructions);
        for _ in 0..num_instructions {
            let mut options = vec![FAIL_NEXT_IO_OPERATION];

            match network_partition {
                Some(_) => options.push(INSTRUCTION_HEAL_NETWORK_PARTITION),
                None => options.push(INSTRUCTION_PARTITION_NETWORK),
            }

            // if failed_nodes.len() < NUM_NODES_IN_CLUSTER {
            //     options.push(INSTRUCTION_FAIL_NODE);
            // }

            // if failed_nodes.len() > 0 {
            //     options.push(INSTRUCTION_RECOVER_NODE);
            // }

            let next_event_time = g.choose(CLOCK_ADVANCE_CHOICES.as_slice()).unwrap();
            clock = clock + *next_event_time;
            let next_event_type = g.choose(&options).unwrap();

            match *next_event_type {
                INSTRUCTION_PARTITION_NETWORK => {
                    let num_partitions = g.choose(&[2, 3, 4, 5]).unwrap();

                    let mut partitions = HashMap::<i32, HashSet<ServerId>>::new();

                    let mut nodes_available: HashSet<_> = NODES.iter().cloned().collect();
                    let mut nodes_selected = HashSet::<ServerId>::new();
                    let mut current_partition = 0;
                    while nodes_available.len() > 0 {
                        let node = *g
                            .choose(&nodes_available.iter().cloned().collect::<Vec<_>>())
                            .unwrap();

                        partitions
                            .entry(current_partition)
                            .or_insert(HashSet::new())
                            .insert(node);

                        nodes_available.remove(&node);
                        nodes_selected.insert(node);

                        current_partition += 1;
                        current_partition %= num_partitions;
                    }

                    let partitions: Vec<_> = partitions.iter().map(|e| e.1).cloned().collect();
                    sequence_of_events.push(SimulatorEvent {
                        time: SimTime::from_millis(clock),
                        action: SimulatorAction::PartitionNetwork(
                            partitions.iter().cloned().collect(),
                        ),
                    });
                    network_partition = Some(partitions);
                }
                INSTRUCTION_HEAL_NETWORK_PARTITION => {
                    sequence_of_events.push(SimulatorEvent {
                        time: SimTime::from_millis(clock),
                        action: SimulatorAction::HealNetworkPartition,
                    });
                    network_partition = None;
                }
                INSTRUCTION_FAIL_NODE => {}
                INSTRUCTION_RECOVER_NODE => {}
                FAIL_NEXT_IO_OPERATION => {
                    FAULT_INJECT_COUNTER.store(1, std::sync::atomic::Ordering::Release);
                }
                _ => panic!("Unknown instruction type"),
            }
        }

        SimInstructionSequence {
            generated_state_changes: sequence_of_events,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        quickcheck::empty_shrinker()
    }
}

fn run_simulation_with_sequence_of_events(
    events: SimInstructionSequence,
    maybe_rng_seed: Option<u64>,
    maybe_log_file_path: Option<&str>,
) {
    set_trigger_function(io_fault_injection_trigger_fn);

    let rng = new_rng(maybe_rng_seed);
    let config = RaftConfig {
        leader_heartbeat_interval: Duration::from_millis(100),
        min_election_timeout_ms: 150,
        max_election_timeout_ms: 300,
    };

    let network = SimNetwork::with_defaults(
        5,
        PacketLossProbability(0.01),
        LatencyMean(5.0),
        LatencyStdDev(2.0),
    );
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap();

    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        sim_log_path(maybe_log_file_path),
    );

    let run_until_time = events
        .generated_state_changes
        .iter()
        .map(|e| e.time)
        .max()
        .unwrap();
    for event in events.generated_state_changes {
        sim.enqueue_event(event);
    }
    sim.enqueue_event(SimulatorEvent {
        time: run_until_time,
        action: SimulatorAction::HealNetworkPartition,
    });

    sim.run_until_time((run_until_time + Duration::from_secs(60)).into());
    assert_eq!(sim.results.was_leader_elected, true);
    drop(sim);
}

#[test]
fn test_with_quickcheck() {
    fn prop(instructions: SimInstructionSequence) {
        run_simulation_with_sequence_of_events(instructions, None, None)
    }

    QuickCheck::new()
        .tests(10)
        .quickcheck(prop as fn(SimInstructionSequence) -> ());
}

#[test]
fn test_with_example_sequence_of_events() {
    let maybe_sequence_of_events: Option<SimInstructionSequence> = None;
    let maybe_rng_seed: Option<u64> = None;
    let maybe_log_file_path: Option<&str> = None;

    if let Some(events) = maybe_sequence_of_events {
        run_simulation_with_sequence_of_events(events, maybe_rng_seed, maybe_log_file_path);
    }
}
