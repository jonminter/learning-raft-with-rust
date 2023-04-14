use std::{collections::HashSet, thread::JoinHandle};

use raft_consensus::{start_raft_in_new_thread, RaftConfig, RaftStateEventCollector, ServerId};
use rand_chacha::ChaCha8Rng;

use super::sim_transport::SimNetworkRaftTransport;

/// A process in the simulation that represents a single server.
/// This runs the Raft algorithm for this simulated server in it's own thread.
/// It uses the provided transport to send and to receive messages from other servers.
pub(crate) struct SimRaftProcess {
    pub(crate) thread_handle: JoinHandle<()>,
}
impl SimRaftProcess {
    pub(crate) fn new(
        server_id: ServerId,
        max_id: u64,
        config: RaftConfig,
        storage_path: String,
        mut rng: ChaCha8Rng,
        transport: SimNetworkRaftTransport,
        event_collector: impl RaftStateEventCollector + 'static,
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
            other_servers,
            storage_path,
            config,
            rng,
            transport,
            event_collector,
        );
        SimRaftProcess {
            thread_handle: raft_thread_handle,
        }
    }

    pub(crate) fn tick(&self) {
        self.thread_handle.thread().unpark();
    }
}
