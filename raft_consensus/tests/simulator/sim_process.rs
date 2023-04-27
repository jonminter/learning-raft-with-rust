use std::{collections::HashSet, thread::JoinHandle};

use raft_consensus::{start_raft_in_new_thread, RaftConfig, RaftStateEventCollector, ServerId};
use rand_chacha::ChaCha8Rng;

use super::{sim_network::SimNetwork, sim_transport::SimNetworkRaftTransportConnector};

/// A process in the simulation that represents a single server.
/// This runs the Raft algorithm for this simulated server in it's own thread.
/// It uses the provided transport to send and to receive messages from other servers.
pub(crate) struct SimRaftProcess<E: RaftStateEventCollector + Clone> {
    server_id: ServerId,
    config: RaftConfig,
    rng: ChaCha8Rng,
    other_servers: HashSet<ServerId>,
    storage_path: String,
    event_collector: E,
    thread_handle: JoinHandle<()>,
}
impl<E: RaftStateEventCollector + Clone + 'static> SimRaftProcess<E> {
    pub(crate) fn new(
        server_id: ServerId,
        max_id: u64,
        config: RaftConfig,
        storage_path: String,
        mut rng: ChaCha8Rng,
        network_to_join: &mut SimNetwork,
        event_collector: E,
    ) -> Self {
        rng.set_stream(server_id.0 as u64);
        assert!(
            server_id.0 <= max_id,
            "Server ID must be less than/equal to max ID"
        );

        let mut other_servers = HashSet::new();
        for s in 0..max_id {
            if s != server_id.0 {
                other_servers.insert(ServerId(s));
            }
        }

        let raft_thread_handle = start_raft_in_new_thread(
            server_id,
            other_servers.clone(),
            storage_path.clone(),
            config,
            rng.clone(),
            network_to_join.join_network_and_take_transport_connector(server_id),
            event_collector.clone(),
        );
        SimRaftProcess {
            server_id,
            rng,
            config,
            other_servers,
            storage_path,
            event_collector,
            thread_handle: raft_thread_handle,
        }
    }

    pub(crate) fn restart_if_needed(&mut self, network_to_join: &mut SimNetwork) {
        if self.thread_handle.is_finished() {
            println!("Restarting server {}...", self.server_id.0);
            self.thread_handle = start_raft_in_new_thread(
                self.server_id,
                self.other_servers.clone(),
                self.storage_path.clone(),
                self.config,
                self.rng.clone(),
                network_to_join.join_network_and_take_transport_connector(self.server_id),
                self.event_collector.clone(),
            );
        }
    }

    pub(crate) fn wake_up_transport_connector(&self) {
        self.thread_handle.thread().unpark();
    }
}
