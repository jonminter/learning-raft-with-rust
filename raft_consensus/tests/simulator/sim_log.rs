use std::{collections::HashMap, fs::File, io::Write, path::{Path, PathBuf}, time::Duration};

use raft_consensus::{
    rpc_messages::{self, ReplyTo, Request, RpcMessage},
    RaftStateEvent, ServerId,
};

use super::common::{SimLogCommand, SimTime, SimulatorEvent};

#[derive(Debug, Clone)]
pub(crate) enum LoggedSimEvent {
    SendOverNetwork(SimTime, RpcMessage<SimLogCommand>),
    DroppedNetworkMessage(SimTime, RpcMessage<SimLogCommand>),
    PartitionNetwork(Vec<Vec<ServerId>>),
    HealNetworkPartition,
    InjectIOFaultEveryNOps(u64),
    RestoreIOFunctioning,
}
impl LoggedSimEvent {
    fn from_sim_event(event: &SimulatorEvent) -> Self {
        match &event.action {
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
            super::common::SimulatorAction::InjectIOFailureEveryNOps(n) => {
                LoggedSimEvent::InjectIOFaultEveryNOps(*n)
            }
            super::common::SimulatorAction::RestoreIOFunctioning => {
                LoggedSimEvent::RestoreIOFunctioning
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct LoggedSimTime(pub(crate) Duration);

#[derive(Debug, Clone)]
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
    log_file: Option<File>,
    events: Vec<SimLogEntry>,
}

fn write_event_to_log_file(log_file: &mut File, event: &SimLogEntry) -> Result<(), std::io::Error> {
    match event {
        SimLogEntry::EventQueued(queued_time, event) => match event {
            LoggedSimEvent::DroppedNetworkMessage(_, _) => {}
            LoggedSimEvent::SendOverNetwork(delivery_time, msg) => match msg {
                RpcMessage::Request(req) => match req {
                    Request::AppendEntries(req) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: SEND AppendEntries from {:?} to {:?} for term {:?} with latency {:?}ms tbd at {:?} (req id: {:?})",
                            queued_time.as_millis(), req.from, req.to, req.term, delivery_time.as_millis() - queued_time.as_millis(), delivery_time.as_millis(), req.request_id
                        )?;
                    }
                    Request::RequestVote(req) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: SEND RequestVote from {:?} to {:?} for term {:?} with latency {:?}ms tbd at {:?} (req id: {:?})",
                            queued_time.as_millis(),
                            req.from,
                            req.to,
                            req.term,
                            delivery_time.as_millis() - queued_time.as_millis(),   
                            delivery_time.as_millis(),
                            req.request_id
                        )?;
                    }
                },
                RpcMessage::Reply(reply) => match reply {
                    ReplyTo::AppendEntries(reply) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: SEND AppendEntriesReply from {:?} to {:?} for term {:?} with latency {:?}ms tbd at {:?} (req id: {:?})",
                            queued_time.as_millis(), reply.from, reply.to, reply.term, delivery_time.as_millis() - queued_time.as_millis(), delivery_time.as_millis(), reply.request_id
                        )?;
                    }
                    ReplyTo::RequestVote(reply) => {
                        writeln!(
                            log_file,
                            "TIME {time:?}ms: SEND RequestVoteReply(vote_granted={vote}) from {from:?} to {to:?} for term {term:?} with latency {latency:?}ms tbd at {delivery_time:?} (req id: {req_id:?})",
                            time=queued_time.as_millis(), vote=reply.vote_granted, from=reply.from, to=reply.to, term=reply.term, latency=delivery_time.as_millis() - queued_time.as_millis(), delivery_time=delivery_time.as_millis(), req_id=reply.request_id
                        )?;
                    }
                },
            },
            LoggedSimEvent::PartitionNetwork(_) => {}
            LoggedSimEvent::HealNetworkPartition => {}
            LoggedSimEvent::InjectIOFaultEveryNOps(_) => {}
            LoggedSimEvent::RestoreIOFunctioning => {}
        },
        SimLogEntry::EventProcessed(time, event) => match event {
            LoggedSimEvent::DroppedNetworkMessage(_, msg) => match msg {
                RpcMessage::Request(req) => match req {
                    rpc_messages::Request::AppendEntries(req) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: DROPPED AppendEntries from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), req.from, req.to, req.term, req.request_id
                        )?;
                    }
                    rpc_messages::Request::RequestVote(req) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: DROPPED RequestVote from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), req.from, req.to, req.term, req.request_id
                        )?;
                    }
                },
                RpcMessage::Reply(reply) => match reply {
                    rpc_messages::ReplyTo::AppendEntries(reply) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: DROPPED AppendEntriesReply from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), reply.from, reply.to, reply.term, reply.request_id
                        )?;
                    }
                    rpc_messages::ReplyTo::RequestVote(reply) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: DROPPED RequestVoteReply from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), reply.from, reply.to, reply.term, reply.request_id
                        )?;
                    }
                },
            },
            LoggedSimEvent::SendOverNetwork(_, msg) => match msg {
                RpcMessage::Request(req) => match req {
                    rpc_messages::Request::AppendEntries(req) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: RECV AppendEntries from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), req.from, req.to, req.term, req.request_id
                        )?;
                    }
                    rpc_messages::Request::RequestVote(req) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: RECV RequestVote from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), req.from, req.to, req.term, req.request_id
                        )?;
                    }
                },
                RpcMessage::Reply(reply) => match reply {
                    rpc_messages::ReplyTo::AppendEntries(reply) => {
                        writeln!(
                            log_file,
                            "TIME {:?}ms: RECV AppendEntriesReply from {:?} to {:?} for term {:?} (req id: {:?})",
                            time.as_millis(), reply.from, reply.to, reply.term,     reply.request_id
                        )?;
                    }
                    rpc_messages::ReplyTo::RequestVote(reply) => {
                        writeln!(
                            log_file,
                            "TIME {time:?}ms: RECV RequestVoteReply(vote_granted={vote:?}) from {from:?} to {to:?} for term {term:?} (req id: {req_id:?})",
                            time=time.as_millis(), vote=reply.vote_granted, from=reply.from, to=reply.to, term=reply.term, req_id=reply.request_id
                        )?;
                    }
                },
            },
            LoggedSimEvent::PartitionNetwork(partitions) => {
                writeln!(
                    log_file,
                    "TIME {:?}ms: PartitionNetwork...",
                    time.as_millis()
                )?;
                for partition in partitions {
                    writeln!(log_file, "    Partition: {:?}", partition)?;
                }
            }
            LoggedSimEvent::HealNetworkPartition => {
                writeln!(
                    log_file,
                    "TIME {:?}ms: HealNetworkPartition",
                    time.as_millis()
                )?;
            }
            LoggedSimEvent::InjectIOFaultEveryNOps(n) => {
                writeln!(
                    log_file,
                    "TIME {:?}ms: InjectIOFaultEveryNOps({:?})",
                    time.as_millis(),
                    n
                )?;
            }
            LoggedSimEvent::RestoreIOFunctioning => {
                writeln!(
                    log_file,
                    "TIME {:?}ms: RestoreIOFunctioning",
                    time.as_millis()
                )?;
            }
        },
        SimLogEntry::ServerStateUpdate(time, server_states) => {
            writeln!(
                log_file,
                "TIME {:?}ms: ServerStates...",
                time.as_millis()
            )?;
            let mut sorted_states = server_states.iter().collect::<Vec<_>>();
            sorted_states.sort_by(|a, b| a.0.cmp(b.0));

            for (server_id, state) in sorted_states.iter() {
                writeln!(
                    log_file,
                    "    Server {:?} is in state {:?}",
                    server_id, state
                )?;
            }
        }
    }
    Ok(())
}

impl SimLog {
    pub(crate) fn new(log_file_path: Option<PathBuf>) -> Self {
        let log_file = log_file_path.map(|p| File::create(p).expect("Could not create log file"));
        Self {
            events: Vec::new(),
            log_file,
        }
    }
    pub(crate) fn push(&mut self, event: SimLogEntry) {
        if let Some( log_file) = &mut self.log_file {
             write_event_to_log_file(log_file, &event)
                .expect("SIM: Could not write to log file");
        }
        self.events.push(event);
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = SimLogEntry> + '_{
        self.events.iter().cloned()
    }
    pub(crate) fn reset(&mut self) {
        self.events.clear();
    }

    pub(crate) fn flush(&mut self) -> Result<(), std::io::Error> {
        if let Some(log_file) = &mut self.log_file {
            log_file.flush()
        } else {
            Ok(())
        }
    }

    
}
