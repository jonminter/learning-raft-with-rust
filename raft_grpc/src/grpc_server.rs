use crate::grpc_transport::TransportInput;
use crate::proto::raft_consensus_server::RaftConsensus;
use crate::proto::{AppendEntriesRequest, AppendEntriesResponse, VoteRequest, VoteResponse};
use raft_consensus::rpc_messages;
use std::thread;
use tokio::sync::{mpsc, oneshot};
use tonic::{Request, Response, Status};

/// Raft gRPC server implementation. Uses the RaftTransportBridge to send incoming requests to the
/// Raft thrad and to receive outgoing requests from the Raft thread.
#[derive(Debug)]
pub struct RaftGrpcServerImpl {
    raft_input_tx: mpsc::UnboundedSender<TransportInput>,
    maybe_transport_thread_handle: Option<thread::JoinHandle<()>>,
}

impl RaftGrpcServerImpl {
    pub fn new(raft_input_tx: mpsc::UnboundedSender<TransportInput>) -> RaftGrpcServerImpl {
        RaftGrpcServerImpl {
            raft_input_tx,
            maybe_transport_thread_handle: None,
        }
    }

    pub fn register_raft_thread(&mut self, transport_thread_handle: thread::JoinHandle<()>) {
        self.maybe_transport_thread_handle = Some(transport_thread_handle);
    }

    /// Send an incoming request to the Raft thread's message queue for processing
    /// The raft thread parks itself while waiting for a new message so we need to unpark it
    /// after sending the new message so it can wake up and resume processing.
    ///
    /// See RaftGrpcTransportBridge::wait_for_next_incoming_message() to see the
    /// implementation of the inverse side, the Raft thread, where it parks the thread while waiting.
    pub fn send_incoming_request_to_transport(
        &self,
        reply_tx: oneshot::Sender<rpc_messages::ReplyTo>,
        incoming_request: rpc_messages::Request<u64>,
    ) {
        self.raft_input_tx
            .send(TransportInput::Request(reply_tx, incoming_request))
            .expect("Failed to send incoming request to gRPC transport!");
        self.maybe_transport_thread_handle
            .as_ref()
            .expect("Transport thread not registered!")
            .thread()
            .unpark();
    }
}

#[tonic::async_trait]
impl RaftConsensus for RaftGrpcServerImpl {
    async fn request_vote(
        &self,
        request: Request<VoteRequest>,
    ) -> Result<Response<VoteResponse>, Status> {
        let vote_req = request.into_inner();

        let (reply_tx, reply_rx) = oneshot::channel();
        self.send_incoming_request_to_transport(
            reply_tx,
            rpc_messages::Request::RequestVote(vote_req.into()),
        );

        let vote_response = reply_rx.await;

        match vote_response {
            Ok(rpc_messages::ReplyTo::RequestVote(vote)) => Ok(Response::new(vote.into())),
            Err(_) => Err(Status::cancelled("Raft state machine shutdown!")),
            _ => unreachable!("BUG ALERT: Unexpected response type, expected SendVote!"),
        }
    }

    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let append_entries_req = request.into_inner();

        let (reply_tx, reply_rx) = oneshot::channel();
        self.send_incoming_request_to_transport(
            reply_tx,
            rpc_messages::Request::AppendEntries(append_entries_req.into()),
        );

        let append_entries_response = reply_rx.await;

        match append_entries_response {
            Ok(rpc_messages::ReplyTo::AppendEntries(append_entries)) => {
                Ok(Response::new(append_entries.into()))
            }
            Err(_) => Err(Status::cancelled("Raft state machine shutdown!")),
            _ => unreachable!("BUG ALERT: Unexpected response type, expected AppendEntries!"),
        }
    }
}
