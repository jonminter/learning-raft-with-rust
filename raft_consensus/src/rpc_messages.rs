use std::fmt::Debug;

use uuid::Uuid;

use super::common::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcMessage<C: LogCommand> {
    Request(Request<C>),
    Reply(ReplyTo),
}

impl<C: LogCommand> RpcMessage<C> {
    pub fn from(&self) -> ServerId {
        match self {
            RpcMessage::Request(request) => request.from(),
            RpcMessage::Reply(reply) => reply.from(),
        }
    }

    pub fn to(&self) -> ServerId {
        match self {
            RpcMessage::Request(request) => request.to(),
            RpcMessage::Reply(reply) => reply.to(),
        }
    }

    pub fn request_id(&self) -> Uuid {
        match self {
            RpcMessage::Request(request) => match request {
                Request::AppendEntries(ae) => ae.request_id,
                Request::RequestVote(rv) => rv.request_id,
            },
            RpcMessage::Reply(reply) => match reply {
                ReplyTo::AppendEntries(ae) => ae.request_id,
                ReplyTo::RequestVote(rv) => rv.request_id,
            },
        }
    }

    pub fn append_entries(append_entries: AppendEntries<C>) -> Self {
        RpcMessage::Request(Request::AppendEntries(append_entries))
    }

    pub fn request_vote(request_vote: RequestVote) -> Self {
        RpcMessage::Request(Request::RequestVote(request_vote))
    }

    pub fn vote(vote: Vote) -> Self {
        RpcMessage::Reply(ReplyTo::RequestVote(vote))
    }

    pub fn ack_append_entries(append_entries_ack: AppendEntriesAck) -> Self {
        RpcMessage::Reply(ReplyTo::AppendEntries(append_entries_ack))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AppendEntries<C: LogCommand> {
    pub request_id: Uuid,
    pub from: ServerId,
    pub to: ServerId,
    pub term: TermIndex,
    pub prev_log_term: TermIndex,
    pub prev_log_index: LogIndex,
    pub entries: Vec<LogEntry<C>>,
    pub leader_commit: LogIndex,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestVote {
    pub request_id: Uuid,
    pub from: ServerId,
    pub to: ServerId,
    pub term: TermIndex,
    pub last_log_index: LogIndex,
    pub last_log_term: TermIndex,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Request<C: LogCommand> {
    AppendEntries(AppendEntries<C>),
    RequestVote(RequestVote),
}
impl<C: LogCommand> Request<C> {
    pub fn from(&self) -> ServerId {
        match self {
            Request::AppendEntries(ae) => ae.from,
            Request::RequestVote(rv) => rv.from,
        }
    }
    pub fn to(&self) -> ServerId {
        match self {
            Request::AppendEntries(ae) => ae.to,
            Request::RequestVote(rv) => rv.to,
        }
    }
    pub fn term(&self) -> TermIndex {
        match self {
            Request::AppendEntries(ae) => ae.term,
            Request::RequestVote(rv) => rv.term,
        }
    }
    pub fn request_id(&self) -> Uuid {
        match self {
            Request::AppendEntries(ae) => ae.request_id,
            Request::RequestVote(rv) => rv.request_id,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AppendEntriesAck {
    pub request_id: Uuid,
    pub from: ServerId,
    pub to: ServerId,
    pub term: TermIndex,
    pub success: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vote {
    pub request_id: Uuid,
    pub from: ServerId,
    pub to: ServerId,
    pub term: TermIndex,
    pub vote_granted: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReplyTo {
    AppendEntries(AppendEntriesAck),
    RequestVote(Vote),
}
impl ReplyTo {
    pub fn from(&self) -> ServerId {
        match self {
            ReplyTo::AppendEntries(ae) => ae.from,
            ReplyTo::RequestVote(rv) => rv.from,
        }
    }
    pub fn to(&self) -> ServerId {
        match self {
            ReplyTo::AppendEntries(ae) => ae.to,
            ReplyTo::RequestVote(rv) => rv.to,
        }
    }
    pub fn term(&self) -> TermIndex {
        match self {
            ReplyTo::AppendEntries(ae) => ae.term,
            ReplyTo::RequestVote(rv) => rv.term,
        }
    }
    pub fn request_id(&self) -> Uuid {
        match self {
            ReplyTo::AppendEntries(ae) => ae.request_id,
            ReplyTo::RequestVote(rv) => rv.request_id,
        }
    }
}
