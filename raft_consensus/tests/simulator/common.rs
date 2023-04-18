use mock_instant::MockClock;
use raft_consensus::{rpc_messages::RpcMessage, ServerId};
use std::{collections::HashSet, ops::Add, time::Duration};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) struct SimLogCommand(pub(crate) u64);

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord, Hash)]
pub(crate) struct SimTime(pub(crate) Duration);
impl SimTime {
    pub(crate) fn checked_sub(&self, other: &Self) -> Option<Duration> {
        self.0.checked_sub(other.0)
    }

    pub(crate) fn from_millis(millis: u64) -> Self {
        SimTime(Duration::from_millis(millis))
    }

    pub(crate) fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    pub(crate) fn now() -> Self {
        SimTime(MockClock::time())
    }
}

impl Add<SimTime> for SimTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        SimTime(self.0 + rhs.0)
    }
}
impl Add<Duration> for SimTime {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        SimTime(self.0 + rhs)
    }
}
impl From<Duration> for SimTime {
    fn from(d: Duration) -> Self {
        SimTime(d)
    }
}
impl From<SimTime> for Duration {
    fn from(t: SimTime) -> Self {
        t.0
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) enum SimulatorAction {
    SendOverNetwork(RpcMessage<SimLogCommand>),
    PartitionNetwork(Vec<HashSet<ServerId>>),
    HealNetworkPartition,
}
#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct SimulatorEvent {
    pub(crate) time: SimTime,
    pub(crate) action: SimulatorAction,
}
impl PartialOrd for SimulatorEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.time.0.cmp(&other.time.0))
    }
}
impl Ord for SimulatorEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.0.cmp(&other.time.0)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub(crate) struct WakeUpAtOrBefore(pub(crate) SimTime);
