use raft_consensus::{rpc_messages::RpcMessage, ServerId};
use std::{collections::HashSet, ops::Add, time::Duration};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) struct SimLogCommand(pub(crate) u64);

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub(crate) struct SimTime(pub(crate) Duration);
impl SimTime {
    pub(crate) fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(SimTime)
    }

    pub(crate) fn from_millis(millis: u64) -> Self {
        SimTime(Duration::from_millis(millis))
    }

    pub(crate) fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }
}
impl Add<ClockAdvance> for SimTime {
    type Output = Self;
    fn add(self, rhs: ClockAdvance) -> Self::Output {
        SimTime(self.0 + rhs.0)
    }
}
impl Add<SimTime> for SimTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        SimTime(self.0 + rhs.0)
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

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum SimulatorAction {
    TickClock,
    SendOverNetwork(RpcMessage<SimLogCommand>),
    PartitionNetwork(Vec<HashSet<ServerId>>),
    HealNetworkPartition,
}
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct SimulatorEvent {
    pub(crate) time: SimTime,
    pub(crate) action: SimulatorAction,
}
impl PartialOrd for SimulatorEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SimulatorEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.0.cmp(&other.time.0).reverse()
    }
}

pub(crate) struct ClockAdvance(pub(crate) Duration);
