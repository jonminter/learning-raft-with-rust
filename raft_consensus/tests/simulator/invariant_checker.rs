use raft_consensus::{RaftNodeState, RaftStateEvent, RaftStateEventCollector, ServerId, TermIndex};
use tracing::info;

use std::{
    collections::{HashMap, HashSet},
    sync::mpsc,
};

use super::{
    common::SimTime,
    sim_log::{SimLog, SimLogEntry},
};

/// Our implementation of raft allows for collection of events that represent the current state
/// of the raft node. This is used to check invariants of the raft algorithm.
/// This is a simple implementation that collects all events and then checks them at each step of the simulation.
/// It uses a channel to send events from the server processes to the aggregator which is used by the
/// simulation to check invariants. The channel is needed since the simulated raft node runs in a separate thread.
#[derive(Clone)]
pub(crate) struct ServerProcessRaftStateEventCollector {
    event_tx: mpsc::Sender<RaftStateEvent>,
}
impl RaftStateEventCollector for ServerProcessRaftStateEventCollector {
    fn push_event(&mut self, event: RaftStateEvent) {
        self.event_tx.send(event).unwrap_or_default();
    }
}

/// This is used by the simulation to check invariants of the raft implementation.
/// This collects all events from all servers, for each incoming event it updates the current state of
/// the server the event is from. It then uses the states of the servers to check that invariants are not violated.
pub(crate) struct InvariantChecker {
    server_states: HashMap<ServerId, RaftStateEvent>,
    event_tx: mpsc::Sender<RaftStateEvent>,
    event_rx: mpsc::Receiver<RaftStateEvent>,
}
impl InvariantChecker {
    pub(crate) fn new() -> Self {
        let (event_tx, event_rx) = mpsc::channel();
        Self {
            server_states: HashMap::new(),
            event_tx,
            event_rx,
        }
    }

    /// Get a new RaftStateEventCollector that can be used to collect events from a server process.
    pub(crate) fn event_collector_for_server(&self) -> ServerProcessRaftStateEventCollector {
        ServerProcessRaftStateEventCollector {
            event_tx: self.event_tx.clone(),
        }
    }

    /// Get the current state of all servers. Returns a cloned copy of the state.
    pub(crate) fn get_current_state(&self) -> HashMap<ServerId, RaftStateEvent> {
        self.server_states
            .iter()
            .map(|(id, state)| (id.clone(), state.clone()))
            .collect()
    }

    pub(crate) fn get_current_leader(&self) -> Option<ServerId> {
        for (id, state) in self.server_states.iter() {
            if let RaftNodeState::Leader = state.current_state {
                return Some(id.clone());
            }
        }
        None
    }

    /// Check that Raft invariants are not violated.
    pub(crate) fn check_invariants(&mut self, time: SimTime, log: &mut SimLog) {
        let old_server_states = self.server_states.clone();
        while let Ok(event) = self.event_rx.try_recv() {
            self.check_state_change_invariants(event);
            self.server_states.insert(event.server_id, event);
        }
        let current_state = self.get_current_state();

        let new_state_has_changes = current_state.iter().any(|(server_id, old_state)| {
            old_server_states
                .get(server_id)
                .map_or(true, |new_state| old_state != new_state)
        });
        let servers_removed = old_server_states
            .keys()
            .any(|id| !current_state.contains_key(id));

        if new_state_has_changes || servers_removed {
            log.push(SimLogEntry::ServerStateUpdate(
                time,
                self.get_current_state(),
            ));
        }

        self.assert_at_most_one_leader_in_term();
    }

    /// Check that when server states change, the new state is valid.
    /// - Term index should always increase
    fn check_state_change_invariants(&self, event: RaftStateEvent) {
        let maybe_old_state = self.server_states.get(&event.server_id);

        if let Some(old_state) = maybe_old_state {
            assert!(
                event.current_term >= old_state.current_term,
                "{:?}: Term index should always increase, old: {old:?}, new: {new:?}",
                event.server_id,
                old = old_state.current_term,
                new = event.current_term
            );
        }
    }

    /// There should only be one leader chosen for a term, this means that:
    /// - Only one node that believes it is the leader for a term
    /// - All nodes should agree on who the leader is for that term
    /// See: <https://homes.cs.washington.edu/~mernst/pub(crate)s/raft-proof-cpp2016.pdf>
    /// Property 2 (Election Safety). There is at most one leader per term.
    fn assert_at_most_one_leader_in_term(&mut self) {
        // TermIndex -> Set nodes that believe they are the leader for that term
        // There should be at max a count of one for each term
        let mut nodes_that_think_they_are_leaders = HashMap::<TermIndex, HashSet<ServerId>>::new();
        // TermIndex -> Set of all nodes where at least one other node believes that node is the leader for this term
        // Each set should contain at most one item
        let mut nodes_that_other_nodes_see_as_leaders =
            HashMap::<TermIndex, HashSet<ServerId>>::new();

        for (_, server_state) in &self.server_states {
            match server_state.current_state {
                RaftNodeState::Leader => {
                    nodes_that_think_they_are_leaders
                        .entry(server_state.current_term)
                        .and_modify(|servers| {
                            servers.insert(server_state.server_id);
                        })
                        .or_insert_with(|| {
                            let mut servers = HashSet::new();
                            servers.insert(server_state.server_id);
                            servers
                        });
                }
                _ => (),
            }

            nodes_that_other_nodes_see_as_leaders
                .entry(server_state.current_term)
                .and_modify(|servers| {
                    if let Some(leader_id) = server_state.leader_for_term {
                        servers.insert(leader_id);
                    }
                })
                .or_insert_with(|| {
                    let mut servers = HashSet::new();
                    if let Some(leader_id) = server_state.leader_for_term {
                        servers.insert(leader_id);
                    }
                    servers
                });
        }

        nodes_that_think_they_are_leaders
            .iter()
            .for_each(|(term, leaders)| {
                assert!(
                    leaders.len() <= 1,
                    "CLUSTER INVARIANT VIOLATED: There should be at most ONE node that believes it is the leader for term {term:?} instead found {count}, leaders are {leaders:?}!",
                    term = term,
                    count = leaders.len(),
                    leaders = leaders
                )
            });
        nodes_that_other_nodes_see_as_leaders.iter().for_each(|(term, leaders)| {
                if leaders.len() > 1 {
                   for (_, server_state) in &self.server_states {
                        server_state.leader_for_term.map(|leader| {
                            info!(
                                "Node {node:?} believes leader for term {term:?} is {leader:?}",
                                node = server_state.server_id,
                                term = server_state.current_term,
                                leader = leader
                            );
                        });
                    }
                }

                assert!(
                    leaders.len() <= 1,
                    "CLUSTER INVARIANT VIOLATED: There should be at most ONE agreed upon leader for term {term:?} instead found {leaders:?}!",
                    term = term,
                    leaders = leaders
                )
            });
    }
}
