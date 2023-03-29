use std::time::Duration;

use super::common::LogCommand;
use super::rpc_messages::ReplyTo;
use super::rpc_messages::Request;
use super::rpc_messages::RpcMessage;

/// A trait that defines the interface for a network transport for Raft.
/// This is used by the Raft node to send and receive messages from other nodes.
/// Using a trait for this allows us to swap a different implementation for testing that uses a simulated network.
pub trait RaftTransportBridge<C: LogCommand>: Send {
    fn wait_for_next_incoming_message(&mut self, max_wait: Duration) -> Option<RpcMessage<C>>;

    fn enqueue_reply(&mut self, reply: ReplyTo);

    fn enqueue_outgoing_request(&mut self, request: Request<C>);
}
