pub use crate::common::*;
pub use crate::default_storage::DefaultPersistentStorage;
use crate::rpc_messages::RpcMessage;
use crate::state_machine::*;
use crate::system_clock;
use rand_chacha::ChaCha8Rng;

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use std::{thread, vec};

use crate::common::RaftTransportConnector;

use tracing::{info, trace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaftNodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaftStateEvent {
    pub server_id: ServerId,
    pub current_state: RaftNodeState,
    pub current_term: TermIndex,
    pub voted_for: Option<ServerId>,
    pub leader_for_term: Option<ServerId>,
}

pub trait RaftStateEventCollector: Send {
    fn push_event(&mut self, event: RaftStateEvent);
}

pub struct NoOpRaftEventCollector;
impl RaftStateEventCollector for NoOpRaftEventCollector {
    fn push_event(&mut self, _event: RaftStateEvent) {}
}

pub fn start_raft_in_new_thread<LC: LogCommand>(
    server_id: ServerId,
    other_servers: HashSet<ServerId>,
    storage_path: String,
    config: RaftConfig,
    mut rng: ChaCha8Rng,
    mut transport_connector: impl RaftTransportConnector<LC> + 'static,
    mut event_collector: impl RaftStateEventCollector + 'static,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start_time = system_clock::now();

        let mut storage = DefaultPersistentStorage::new(Path::new(&storage_path));

        let (mut state, first_election_timeout) =
            Node::new(server_id, other_servers, &config, &mut rng);
        info!(
            "{:?}: Starting raft node with state: {:?}, term: {:?}",
            server_id,
            match state {
                Node::Follower(_) => RaftNodeState::Follower,
                Node::Candidate(_) => RaftNodeState::Candidate,
                Node::Leader(_) => RaftNodeState::Leader,
            },
            storage.current_term(),
        );

        let mut max_wait_time = first_election_timeout.0;
        loop {
            trace!(
                "Waiting {:?}ms for next message at time {:?}...",
                max_wait_time.as_millis(),
                start_time.elapsed().as_millis(),
            );

            let time_before_waiting = system_clock::now();
            let maybe_next_message =
                transport_connector.wait_for_next_incoming_message(max_wait_time);

            trace!(
                "Got next message: {:?} after waiting for {:?}, time is now {:?}",
                maybe_next_message,
                time_before_waiting.elapsed().as_millis(),
                start_time.elapsed().as_millis(),
            );

            let (mut new_state, mut tick_actions) = match state.next(
                Event::Tick(system_clock::now()),
                &mut storage,
                &config,
                &mut rng,
            ) {
                Ok((new_state, actions)) => (new_state, actions),
                Err(_) => {
                    info!("Persistent storage error, shutting down raft thread...");
                    return;
                }
            };

            if let Err(_) = maybe_next_message {
                info!("Transport shutdown, shutting down raft thread...");
                return;
            }

            let mut actions_after_processing_message =
                if let Ok(Some(incoming_message)) = maybe_next_message {
                    let actions;
                    (new_state, actions) = match new_state.next(
                        Event::IncomingRpc(incoming_message),
                        &mut storage,
                        &config,
                        &mut rng,
                    ) {
                        Ok((new_state, actions)) => (new_state, actions),
                        Err(_) => {
                            info!("Persistent storage error, shutting down raft thread...");
                            return;
                        }
                    };
                    actions
                } else {
                    vec![]
                };

            max_wait_time = max_wait_time
                .checked_sub(time_before_waiting.elapsed())
                .unwrap_or(Duration::from_millis(0));

            for action in tick_actions
                .drain(..)
                .chain(actions_after_processing_message.drain(..))
            {
                match action {
                    Action::OutgoingRpc(RpcMessage::Request(r)) => {
                        if let Err(_) = transport_connector.enqueue_outgoing_request(r) {
                            info!("Transport shutdown, shutting down raft thread...");
                            return;
                        }
                    }
                    Action::OutgoingRpc(RpcMessage::Reply(message)) => {
                        if let Err(_) = transport_connector.enqueue_reply(message) {
                            info!("Transport shutdown, shutting down raft thread...");
                            return;
                        }
                    }
                    Action::SetNextTimeout(timer_duration) => {
                        trace!("Resetting wait timeout to duration {:?}", timer_duration);
                        max_wait_time = timer_duration;
                    }
                    Action::ApplyLogEntries(_) => todo!(),
                }
            }

            event_collector.push_event(RaftStateEvent {
                server_id,
                current_state: match new_state {
                    Node::Follower(_) => RaftNodeState::Follower,
                    Node::Candidate(_) => RaftNodeState::Candidate,
                    Node::Leader(_) => RaftNodeState::Leader,
                },
                current_term: storage.current_term(),
                voted_for: storage.vote_for_current_term(),
                leader_for_term: match &new_state {
                    Node::Leader(_) => Some(server_id),
                    Node::Follower(follower) => follower.inner.leader_id,
                    _ => None,
                },
            });

            state = new_state;
        }
    })
}
