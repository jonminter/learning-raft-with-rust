/// Tests consensus with simulator
use crate::simulator::{
    common::{SimTime, SimulatorAction, SimulatorEvent},
    sim_network::{LatencyMean, LatencyStdDev, PacketLossProbability, SimNetwork},
    ClusterSim,
};
use raft_consensus::{RaftConfig, ServerId};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::{collections::HashSet, time::Duration};
use tempfile::TempDir;
use tracing::info;
mod simulator;
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
            info!("====================================");
            info!("RNG SEED FOR TESTS: {seed}", seed = seed);
            info!("====================================");
            ChaCha8Rng::seed_from_u64(seed)
        }
    }
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
    let pwd = std::env::current_dir().unwrap();
    let log_file = pwd.join("..").join("should_elect_leader.log");
    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        log_file.as_path(),
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
    let pwd = std::env::current_dir().unwrap();
    let log_file = pwd.join("..").join("should_elect_leader_with_quorum.log");

    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        log_file.as_path(),
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
    let log_file = pwd
        .join("..")
        .join("should_not_elect_leader_without_quorum.log");

    let mut sim = ClusterSim::new(
        5,
        network,
        config,
        rng,
        temp_dir_path.into(),
        log_file.as_path(),
    );

    info!("Current sim time is {time:?}", time = SimTime::now());
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
