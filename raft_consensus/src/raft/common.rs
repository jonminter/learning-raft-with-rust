use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub struct ServerId(pub u64);

pub trait LogCommand: Debug + Clone + Send + Eq + PartialEq {}
impl<T> LogCommand for T where T: Debug + Clone + Send + Eq + PartialEq {}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct LogIndex(pub u64);

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub struct TermIndex(pub u64);
impl TermIndex {
    pub fn increment(&mut self) -> Self {
        TermIndex(self.0 + 1)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LogEntry<T: LogCommand> {
    pub index: LogIndex,
    pub term: TermIndex,
    pub command: T,
}

#[derive(Debug, Clone, Copy)]
pub struct RaftConfig {
    pub leader_heartbeat_interval: Duration,
    pub min_election_timeout_ms: u32,
    pub max_election_timeout_ms: u32,
}

pub trait PersistentStorage<C: LogCommand>: Send {
    fn current_term(&self) -> TermIndex;
    fn voted_for(&self) -> Option<(TermIndex, ServerId)>;

    fn update_term(&mut self, term: TermIndex) -> &mut Self;
    fn record_vote(&mut self, voted_for: ServerId) -> &mut Self;

    fn last_entry_index(&self) -> Option<LogIndex>;
    fn has_entry(self, index: LogIndex, term: TermIndex) -> bool;

    fn append(&mut self, entries: Vec<LogEntry<C>>) -> &mut Self;

    fn sync(&mut self);
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
