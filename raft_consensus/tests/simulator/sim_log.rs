use std::{collections::HashMap, fs::File, io::Write, path::Path, time::Duration};

use raft_consensus::{
    rpc_messages::{self, ReplyTo, Request, RpcMessage},
    RaftStateEvent, ServerId,
};

use super::common::{SimLogCommand, SimTime, SimulatorEvent};

#[derive(Debug)]
pub(crate) enum LoggedSimEvent {
    Tick,
    SendOverNetwork(SimTime, RpcMessage<SimLogCommand>),
    PartitionNetwork(Vec<Vec<ServerId>>),
    HealNetworkPartition,
}
impl LoggedSimEvent {
    fn from_sim_event(event: &SimulatorEvent) -> Self {
        match &event.action {
            super::common::SimulatorAction::TickClock => LoggedSimEvent::Tick,
            super::common::SimulatorAction::SendOverNetwork(msg) => {
                LoggedSimEvent::SendOverNetwork(event.time, msg.clone())
            }
            super::common::SimulatorAction::PartitionNetwork(partitions) => {
                LoggedSimEvent::PartitionNetwork(
                    partitions
                        .iter()
                        .map(|p| p.iter().copied().collect())
                        .collect(),
                )
            }
            super::common::SimulatorAction::HealNetworkPartition => {
                LoggedSimEvent::HealNetworkPartition
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct LoggedSimTime(pub(crate) Duration);

#[derive(Debug)]
pub(crate) enum SimLogEntry {
    EventQueued(SimTime, LoggedSimEvent),
    EventProcessed(SimTime, LoggedSimEvent),
    ServerStateUpdate(SimTime, HashMap<ServerId, RaftStateEvent>),
}
impl SimLogEntry {
    pub(crate) fn event_queued(queued_time: SimTime, event: &SimulatorEvent) -> Self {
        SimLogEntry::EventQueued(queued_time, LoggedSimEvent::from_sim_event(event))
    }
    pub(crate) fn event_processed(process_time: SimTime, event: &SimulatorEvent) -> Self {
        SimLogEntry::EventProcessed(process_time, LoggedSimEvent::from_sim_event(event))
    }
}

pub(crate) struct SimLog {
    log_file: File,
    events: Vec<SimLogEntry>,
}
impl SimLog {
    pub(crate) fn new(log_file_path: &Path) -> Self {
        let log_file = File::create(log_file_path).expect("Could not create log file");
        Self {
            events: Vec::new(),
            log_file,
        }
    }
    pub(crate) fn push(&mut self, event: SimLogEntry) {
        self.append_to_file(&event)
            .expect("SIM: Could not write to log file");
        self.events.push(event);
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &SimLogEntry> {
        self.events.iter()
    }
    pub(crate) fn reset(&mut self) {
        self.events.clear();
    }

    fn append_to_file(&mut self, event: &SimLogEntry) -> Result<(), std::io::Error> {
        match event {
            SimLogEntry::EventQueued(time, event) => match event {
                LoggedSimEvent::Tick => {}
                LoggedSimEvent::SendOverNetwork(delivery_time, msg) => match msg {
                    RpcMessage::Request(req) => match req {
                        Request::AppendEntries(req) => {
                            writeln!(
                                self.log_file,
                                "TIME {:?}ms: SEND AppendEntries from {:?} to {:?} for term {:?} with latency {:?}ms",
                                time.as_millis(), req.from, req.to, req.term, delivery_time.as_millis() - time.as_millis()
                            )?;
                        }
                        Request::RequestVote(req) => {
                            writeln!(
                                self.log_file,
                                "TIME {:?}ms: SEND RequestVote from {:?} to {:?} for term {:?} with latency {:?}ms",
                                time.as_millis(),
                                req.from,
                                req.to,
                                req.term,
                                delivery_time.as_millis() - time.as_millis()
                            )?;
                        }
                    },
                    RpcMessage::Reply(reply) => match reply {
                        ReplyTo::AppendEntries(reply) => {
                            writeln!(
                                self.log_file,
                                "TIME {:?}ms: SEND AppendEntriesReply from {:?} to {:?} for term {:?} with latency {:?}ms",
                                time.as_millis(), reply.from, reply.to, reply.term, delivery_time.as_millis() - time.as_millis()
                            )?;
                        }
                        ReplyTo::RequestVote(reply) => {
                            writeln!(
                                self.log_file,
                                "TIME {:?}ms: SEND RequestVoteReply from {:?} to {:?} for term {:?} with latency {:?}ms",
                                time.as_millis(), reply.from, reply.to, reply.term, delivery_time.as_millis() - time.as_millis()
                            )?;
                        }
                    },
                },
                LoggedSimEvent::PartitionNetwork(_) => {}
                LoggedSimEvent::HealNetworkPartition => {}
            },
            SimLogEntry::EventProcessed(time, event) => {
                match event {
                    LoggedSimEvent::Tick => {}
                    LoggedSimEvent::SendOverNetwork(_, msg) => {
                        match msg {
                            RpcMessage::Request(req) => match req {
                                rpc_messages::Request::AppendEntries(req) => {
                                    writeln!(
                                self.log_file,
                                "TIME {:?}ms: RECV AppendEntries from {:?} to {:?} for term {:?}",
                                time.as_millis(), req.from, req.to, req.term
                            )?;
                                }
                                rpc_messages::Request::RequestVote(req) => {
                                    writeln!(
                                self.log_file,
                                "TIME {:?}ms: RECV RequestVote from {:?} to {:?} for term {:?}",
                                time.as_millis(), req.from, req.to, req.term
                            )?;
                                }
                            },
                            RpcMessage::Reply(reply) => match reply {
                                rpc_messages::ReplyTo::AppendEntries(reply) => {
                                    writeln!(
                                self.log_file,
                                "TIME {:?}ms: RECV AppendEntriesReply from {:?} to {:?} for term {:?}",
                                time.as_millis(), reply.from, reply.to, reply.term
                            )?;
                                }
                                rpc_messages::ReplyTo::RequestVote(reply) => {
                                    writeln!(
                                self.log_file,
                                "TIME {:?}ms: RECV RequestVoteReply from {:?} to {:?} for term {:?}",
                                time.as_millis(), reply.from, reply.to, reply.term
                            )?;
                                }
                            },
                        }
                    }
                    LoggedSimEvent::PartitionNetwork(partitions) => {
                        writeln!(
                            self.log_file,
                            "TIME {:?}ms: PartitionNetwork...",
                            time.as_millis()
                        )?;
                        for partition in partitions {
                            writeln!(self.log_file, "    Partition: {:?}", partition)?;
                        }
                    }
                    LoggedSimEvent::HealNetworkPartition => {
                        writeln!(
                            self.log_file,
                            "TIME {:?}ms: HealNetworkPartition",
                            time.as_millis()
                        )?;
                    }
                }
            }
            SimLogEntry::ServerStateUpdate(time, server_states) => {
                writeln!(
                    self.log_file,
                    "TIME {:?}ms: ServerStates...",
                    time.as_millis()
                )?;
                let mut sorted_states = server_states.iter().collect::<Vec<_>>();
                sorted_states.sort_by(|a, b| a.0.cmp(b.0));

                for (server_id, state) in sorted_states.iter() {
                    writeln!(
                        self.log_file,
                        "    Server {:?} is in state {:?}",
                        server_id, state
                    )?;
                }
            }
        }
        Ok(())
    }
}
