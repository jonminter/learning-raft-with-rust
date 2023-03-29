use crate::grpc_server::RaftGrpcServerImpl;
use crate::proto;
use crate::proto::raft_consensus_client::RaftConsensusClient;
pub use raft_consensus::rpc_messages;
use raft_consensus::rpc_messages::RpcMessage;
use raft_consensus::system_clock;
use raft_consensus::ServerId;
use tonic::transport::Channel;

use raft_consensus::transport::RaftTransportBridge;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use tracing::{info, trace};
use uuid::Uuid;

use tonic::{Request, Status};

use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum TransportInput {
    Request(
        oneshot::Sender<rpc_messages::ReplyTo>,
        rpc_messages::Request<u64>,
    ),
    Reply(rpc_messages::ReplyTo),
}

#[derive(Debug)]
pub struct RaftGrpcTransportBridge {
    raft_input_rx: mpsc::UnboundedReceiver<TransportInput>,
    raft_output_tx: mpsc::UnboundedSender<rpc_messages::Request<u64>>,
    thread_handle: Option<thread::Thread>,
    reply_channels: HashMap<Uuid, oneshot::Sender<rpc_messages::ReplyTo>>,
}
impl RaftGrpcTransportBridge {
    pub fn new(
        raft_input_rx: mpsc::UnboundedReceiver<TransportInput>,
        raft_output_tx: mpsc::UnboundedSender<rpc_messages::Request<u64>>,
    ) -> RaftGrpcTransportBridge {
        RaftGrpcTransportBridge {
            raft_input_rx,
            raft_output_tx,
            thread_handle: None,
            reply_channels: HashMap::new(),
        }
    }
}

impl RaftTransportBridge<u64> for RaftGrpcTransportBridge {
    /// Raft thread calls this method to retrieve the next message to process
    /// This method blocks the Raft thread until a message is available or the max_wait time has elapsed
    /// If the max_wait time has elapsed without receiving a message, this method returns None
    fn wait_for_next_incoming_message(
        &mut self,
        max_wait: std::time::Duration,
    ) -> Option<RpcMessage<u64>> {
        let current_thread = thread::current();
        let current_thread_id = current_thread.id();
        let saved_handle = self.thread_handle.get_or_insert(current_thread);

        assert_eq!(
            saved_handle.id(),
            current_thread_id,
            "Can only wait for next gRPC transport message from a single thread!"
        );

        let started_waiting_at = system_clock::now();

        loop {
            match self.raft_input_rx.try_recv() {
                Ok(TransportInput::Request(reply_tx, message)) => {
                    self.reply_channels.insert(message.request_id(), reply_tx);
                    return Some(RpcMessage::Request(message));
                }
                Ok(TransportInput::Reply(reply)) => {
                    return Some(RpcMessage::Reply(reply));
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    let time_waited = system_clock::now() - started_waiting_at;
                    if time_waited >= max_wait {
                        break None;
                    }
                    thread::park_timeout(max_wait - time_waited);
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    panic!("Raft gRPC transport bridge disconnected!");
                }
            }
        }
    }

    fn enqueue_reply(&mut self, reply: rpc_messages::ReplyTo) {
        self.reply_channels
            .remove(&reply.request_id())
            .expect("BUG ALERT: No reply channel for this request!")
            .send(reply)
            .expect("gRPC transport repy receiver disconnected!");
    }

    fn enqueue_outgoing_request(&mut self, request: rpc_messages::Request<u64>) {
        self.raft_output_tx
            .send(request)
            .expect("gRPC transport outgoing message receiver disconnected!");
    }
}

async fn start_outgoing_message_sender(
    mut server_grpc_clients: HashMap<ServerId, RaftConsensusClient<Channel>>,
    raft_input_tx: mpsc::UnboundedSender<TransportInput>,
    mut raft_output_rx: mpsc::UnboundedReceiver<rpc_messages::Request<u64>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        info!("Starting gRPC transport message sender task...");
        loop {
            if let Some(message) = raft_output_rx.recv().await {
                match message {
                    rpc_messages::Request::RequestVote(vote_req) => {
                        let vote_req: proto::VoteRequest = vote_req.into();
                        let to = ServerId(vote_req.to);

                        let client = server_grpc_clients
                            .get_mut(&to)
                            .expect("BUG ALERT: No gRPC client for this server!");

                        let _ = client
                            .request_vote(Request::new(vote_req))
                            .await
                            .and_then(|response| {
                                raft_input_tx
                                    .send(TransportInput::Reply(
                                        rpc_messages::ReplyTo::RequestVote(
                                            response.into_inner().into(),
                                        ),
                                    ))
                                    .map(|_| ())
                                    .map_err(|e| match e {
                                        mpsc::error::SendError(_) => Status::internal(
                                            "Raft gRPC transport bridge disconnected!",
                                        ),
                                    })
                            })
                            .map_err(|e| {
                                trace!("Failed to send request vote request: {:?}", e);
                            });
                    }
                    rpc_messages::Request::AppendEntries(append_entries_req) => {
                        let append_entries_req: proto::AppendEntriesRequest =
                            append_entries_req.into();
                        let to = ServerId(append_entries_req.to);

                        let client = server_grpc_clients
                            .get_mut(&to)
                            .expect("BUG ALERT: No gRPC client for this server!");

                        let _ = client
                                .append_entries(Request::new(append_entries_req))
                                .await
                                .and_then(|response| {
                                    raft_input_tx
                                        .send(TransportInput::Reply(
                                            rpc_messages::ReplyTo::AppendEntries(
                                                response.into_inner().into(),
                                            ),
                                        ))
                                        .map(|_| ())
                                        .map_err(|e| match e {
                                            mpsc::error::SendError(_) => {
                                                Status::internal("Raft gRPC transport bridge is disconnected, dropping message!")
                                            }
                                        })
                                })
                                .map_err(|e| {
                                    trace!("Failed to send append entries request to {:?}: {:?}", to, e);
                                });
                    }
                }
            } else {
                info!("Raft gRPC transport message sender exiting, raft state machine receiver disconnected/closed!");
                break;
            }
        }
    })
}

pub struct RaftGrpcTransport {
    pub grpc_server: RaftGrpcServerImpl,
    pub transport_bridge: RaftGrpcTransportBridge,
    pub message_sender_task: tokio::task::JoinHandle<()>,
}
impl RaftGrpcTransport {
    pub async fn start_grpc_transport(
        server_id: ServerId,
        server_addresses: HashMap<ServerId, SocketAddr>,
    ) -> RaftGrpcTransport {
        let mut server_grpc_clients: HashMap<ServerId, RaftConsensusClient<Channel>> =
            HashMap::new();
        for (other_server_id, server_address) in server_addresses {
            if other_server_id != server_id {
                let channel = Channel::from_shared(format!("http://{}", server_address))
                    .expect("Failed to create channel")
                    .connect_lazy();
                server_grpc_clients
                    .insert(other_server_id, RaftConsensusClient::new(channel.clone()));
            }
        }

        // Message queues between raft thread and gRPC transport
        // Each runs in a separate thread so need to communicate with channels
        let (raft_input_tx, raft_input_rx) = mpsc::unbounded_channel::<TransportInput>();
        let (raft_output_tx, raft_output_rx) =
            mpsc::unbounded_channel::<rpc_messages::Request<u64>>();

        let transport_bridge = RaftGrpcTransportBridge::new(raft_input_rx, raft_output_tx.clone());
        let grpc_server = RaftGrpcServerImpl::new(raft_input_tx.clone());

        // Outbound RPC messages from raft thread are sent here
        let message_sender =
            start_outgoing_message_sender(server_grpc_clients, raft_input_tx, raft_output_rx).await;
        RaftGrpcTransport {
            grpc_server,
            transport_bridge,
            message_sender_task: message_sender,
        }
    }
}
