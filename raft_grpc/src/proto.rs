use raft_consensus::rpc_messages;
use raft_consensus::{LogIndex, ServerId, TermIndex};
use tonic;
use uuid::Uuid;

tonic::include_proto!("raft"); // The string specified here must match the proto package name

// These convert the protobuf representation of the messages into the form needed for the Raft consensus module.
// The module does not make any assumptions about the transport layer, so it uses it's own types to represent the messages received from the network.

impl From<VoteRequest> for rpc_messages::RequestVote {
    fn from(vote_request: VoteRequest) -> Self {
        rpc_messages::RequestVote {
            request_id: Uuid::parse_str(&vote_request.request_id).expect("Invalid UUID!"),
            from: ServerId(vote_request.from),
            to: ServerId(vote_request.to),
            term: TermIndex(vote_request.term),
            last_log_index: LogIndex(vote_request.last_log_index),
            last_log_term: TermIndex(vote_request.last_log_term),
        }
    }
}
impl From<VoteResponse> for rpc_messages::Vote {
    fn from(vote_response: VoteResponse) -> Self {
        rpc_messages::Vote {
            request_id: Uuid::parse_str(&vote_response.request_id).expect("Invalid UUID!"),
            from: ServerId(vote_response.from),
            to: ServerId(vote_response.to),
            term: TermIndex(vote_response.term),
            vote_granted: vote_response.vote_granted,
        }
    }
}
impl From<AppendEntriesRequest> for rpc_messages::AppendEntries<u64> {
    fn from(append_entries_request: AppendEntriesRequest) -> Self {
        rpc_messages::AppendEntries {
            request_id: Uuid::parse_str(&append_entries_request.request_id).expect("Invalid UUID!"),
            from: ServerId(append_entries_request.from),
            to: ServerId(append_entries_request.to),
            term: TermIndex(append_entries_request.term),
            entries: append_entries_request
                .entries
                .into_iter()
                .map(|entry| raft_consensus::LogEntry {
                    term: TermIndex(entry.term),
                    index: LogIndex(entry.log_index),
                    command: entry
                        .command
                        .map(|c| match c {
                            log_entry::Command::ApplicationCommand(ApplicationCommand {
                                serialized,
                            }) => u64::from_be_bytes(
                                serialized.try_into().expect("Invalid application command!"),
                            ),
                            _ => panic!("Unexpected command!"),
                        })
                        .expect("No command"),
                })
                .collect(),
            prev_log_index: LogIndex(append_entries_request.prev_log_index),
            prev_log_term: TermIndex(append_entries_request.prev_log_term),
            leader_commit: LogIndex(append_entries_request.leader_commit_index),
        }
    }
}
impl From<AppendEntriesResponse> for rpc_messages::AppendEntriesAck {
    fn from(append_entries_response: AppendEntriesResponse) -> Self {
        rpc_messages::AppendEntriesAck {
            request_id: Uuid::parse_str(&append_entries_response.request_id)
                .expect("Invalid UUID!"),
            from: ServerId(append_entries_response.from),
            to: ServerId(append_entries_response.to),
            term: TermIndex(append_entries_response.term),
            success: append_entries_response.added_entries_successfully,
        }
    }
}

impl From<rpc_messages::RequestVote> for VoteRequest {
    fn from(vote_request: rpc_messages::RequestVote) -> Self {
        VoteRequest {
            request_id: vote_request.request_id.to_string(),
            from: vote_request.from.0,
            to: vote_request.to.0,
            term: vote_request.term.0,
            last_log_index: vote_request.last_log_index.0,
            last_log_term: vote_request.last_log_term.0,
        }
    }
}

impl From<rpc_messages::Vote> for VoteResponse {
    fn from(vote_response: rpc_messages::Vote) -> Self {
        VoteResponse {
            request_id: vote_response.request_id.to_string(),
            from: vote_response.from.0,
            to: vote_response.to.0,
            term: vote_response.term.0,
            vote_granted: vote_response.vote_granted,
        }
    }
}

impl From<rpc_messages::AppendEntries<u64>> for AppendEntriesRequest {
    fn from(append_entries_request: rpc_messages::AppendEntries<u64>) -> Self {
        AppendEntriesRequest {
            request_id: append_entries_request.request_id.to_string(),
            from: append_entries_request.from.0,
            to: append_entries_request.to.0,
            term: append_entries_request.term.0,
            entries: append_entries_request
                .entries
                .into_iter()
                .map(|entry| LogEntry {
                    term: entry.term.0,
                    log_index: entry.index.0,
                    command: Some(log_entry::Command::ApplicationCommand(ApplicationCommand {
                        serialized: entry.command.to_be_bytes().to_vec(),
                    })),
                })
                .collect(),
            prev_log_index: append_entries_request.prev_log_index.0,
            prev_log_term: append_entries_request.prev_log_term.0,
            leader_commit_index: append_entries_request.leader_commit.0,
        }
    }
}

impl From<rpc_messages::AppendEntriesAck> for AppendEntriesResponse {
    fn from(append_entries_response: rpc_messages::AppendEntriesAck) -> Self {
        AppendEntriesResponse {
            request_id: append_entries_response.request_id.to_string(),
            from: append_entries_response.from.0,
            to: append_entries_response.to.0,
            term: append_entries_response.term.0,
            added_entries_successfully: append_entries_response.success,
        }
    }
}
