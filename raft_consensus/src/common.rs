use crate::rpc_messages::{ReplyTo, Request, RpcMessage};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
/// A unique identifier for a server in the cluster.
pub struct ServerId(pub u64);

/// A trait that defines the interface for a log command.
pub trait LogCommand: Debug + Clone + Send + Eq + PartialEq {}
impl<T> LogCommand for T where T: Debug + Clone + Send + Eq + PartialEq {}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
/// The index of a log entry.
pub struct LogIndex(pub u64);

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
/// The term of a log entry.
pub struct TermIndex(pub u64);
impl TermIndex {
    /// Increments the term index by 1.
    pub fn increment(&mut self) -> Self {
        TermIndex(self.0 + 1)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// A log entry in the Raft log.
pub struct LogEntry<T: LogCommand> {
    /// The index of the log entry.
    pub index: LogIndex,
    /// The term of the log entry.
    pub term: TermIndex,
    /// The command that was applied to the state machine to produce this log entry.
    pub command: T,
}

#[derive(Debug, Clone, Copy)]
/// The configuration for a Raft node.
pub struct RaftConfig {
    /// The amount of time that a leader will wait before sending a heartbeat to its followers.
    pub leader_heartbeat_interval: Duration,
    /// The minimum amount of time that a follower will wait before becoming a candidate.
    pub min_election_timeout_ms: u32,
    /// The maximum amount of time that a follower will wait before becoming a candidate.
    pub max_election_timeout_ms: u32,
}

#[derive(Debug, Clone, Copy)]
/// Defines errors that can occur when interacting with the persistent storage layer.
pub enum PersistentStorageError {
    /// An error occurred while reading from/writing to disk.
    IoError,
    /// An error occurred while serializing/deserializing data.
    SerdeError,
}

/// A trait that defines the interface for a persistent storage layer for Raft.
pub trait PersistentStorage<C: LogCommand>: Send {
    /// Returns the current term of the Raft node.
    fn current_term(&self) -> TermIndex;
    /// Returns the server that the Raft node voted for in the current term.
    fn voted_for(&self) -> Option<ServerId>;

    /// Updates the current term of the Raft node.
    fn update_term(&mut self, term: TermIndex) -> &mut Self;
    /// Updates the current term of the Raft node.
    fn record_vote(&mut self, voted_for: ServerId) -> &mut Self;

    /// Returns the log index of the last entry in the log.
    fn last_entry_index(&self) -> Option<LogIndex>;
    /// Returns true if the log contains an entry with the given index and term.
    fn has_entry(self, index: LogIndex, term: TermIndex) -> bool;

    /// Appends the given entries to the log.
    fn append(&mut self, entries: Vec<LogEntry<C>>) -> &mut Self;

    /// Writes/fsyncs any pending changes to disk.
    fn sync(&mut self) -> Result<(), PersistentStorageError>;
}

#[derive(Debug)]
/// Enum of errors that can originate from the Raft transport code
pub enum RaftTransportError {
    /// The transport was shutdown.
    TransportShutdown,
}

/// A trait that defines the interface for a network transport for Raft.
/// This is used by the Raft node to send and receive messages from other nodes.
/// Using a trait for this allows us to swap a different implementation for testing that uses a simulated network.
pub trait RaftTransportBridge<C: LogCommand>: Send {
    /// Returns the next incoming message from the network.
    fn wait_for_next_incoming_message(
        &mut self,
        max_wait: Duration,
    ) -> Result<Option<RpcMessage<C>>, RaftTransportError>;

    /// Enqueues a reply to be sent to the given server.
    fn enqueue_reply(&mut self, reply: ReplyTo) -> Result<(), RaftTransportError>;

    /// Enqueues a request to be sent to the given server.
    fn enqueue_outgoing_request(&mut self, request: Request<C>) -> Result<(), RaftTransportError>;
}

/// A trait that defines the interface for a state machine that can be used with Raft.
/// The state machine is responsible for applying commands to its state and returning
/// an error if the command cannot be queued for applying to the application's state machine.
///
/// Additionally, the state machine must be able to return the index of the last command successfully applied.
/// This is used by Raft to determine if it can commit a new entry
trait ApplicationThatNeedsConsensus: Send {
    type Command: LogCommand;
    type Error: Debug + Clone + Send + Eq + PartialEq;

    fn apply(&mut self, log_index: LogIndex, command: Self::Command) -> Result<(), Self::Error>;
    fn last_applied_index(&self) -> LogIndex;
}
