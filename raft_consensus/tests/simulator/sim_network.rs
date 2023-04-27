// TODO: Network simulation is overly complicated
// Instead of trying to implement the transport layer for the entire network
// Create one for each server process, can take the outgoing messages and put on
// simulator queue. Can handle replies inside of the transport object, when
// the simulator delivers an incoming request at that point we can
// create the oneshot channel for the reply, can have a function that gets
// polled by the simulator to check for replies that are ready to be sent
// Also maybe we can use Futures to make this easier? Well maybe not easier
// but make the interface make a little more sense, encapsulate the
// channels a little better

use std::{
    collections::{HashMap, HashSet},
    sync::mpsc,
    time::Duration,
};

use mock_instant::MockClock;
use raft_consensus::{rpc_messages::RpcMessage, LogCommand, ServerId};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Bernoulli, Distribution, LogNormal};
use tracing::{debug, trace};

use super::{
    common::{SimLogCommand, SimTime, WakeUpAtOrBefore},
    sim_log::{LoggedSimEvent, SimLog, SimLogEntry},
    sim_transport::SimNetworkRaftTransportConnector,
};

use rand_distr::num_traits::ToPrimitive;

#[derive(Debug, Clone)]
pub(crate) struct PacketLossProbability(pub(crate) f64);
#[derive(Debug, Clone)]
pub(crate) struct LatencyMean(pub(crate) f64);
#[derive(Debug, Clone)]
pub(crate) struct LatencyStdDev(pub(crate) f64);

pub(crate) struct NetworkConnectionQuality {
    /// Probability that a message is dropped
    packet_loss: Bernoulli,
    /// Latency is calculated with a log-normal distribution
    latency: LogNormal<f64>,
}

struct NetworkNode<C: LogCommand> {
    incoming_message_tx: mpsc::Sender<RpcMessage<C>>,
}

/// Models a network with packet loss and latency, uses Bernoulli distribution for packet loss and log-normal distribution for latency
pub(crate) struct SimNetwork {
    pub(crate) server_ids: HashSet<ServerId>,
    /// Servers in network, map of IDs to network nodes (which contain the transport and incoming message channel)
    servers: HashMap<ServerId, NetworkNode<SimLogCommand>>,
    /// Map of server IDs to probability of packet loss, mean latency, std dev for latency ((server_id, server_id) -> (probability of message being dropped, mean latency, standard deviation)
    connections: HashMap<(ServerId, ServerId), NetworkConnectionQuality>,
    /// Sender side of channel that sends outgoing messages from server process to network to be delivered to other servers
    outbound_message_tx: mpsc::Sender<RpcMessage<SimLogCommand>>,
    /// Receiver side of channel that receives outgoing messages from the server processes
    outbound_message_rx: mpsc::Receiver<RpcMessage<SimLogCommand>>,
    /// Used by server transport connector to register a wake up request
    timer_tx: mpsc::Sender<WakeUpAtOrBefore>,
    /// Used to retrieve wake up requests
    maybe_timer_rx: Option<mpsc::Receiver<WakeUpAtOrBefore>>,
}

impl SimNetwork {
    /// Creates a new network with the given connections
    /// `network_connections` - Map of server IDs to probability of packet loss, mean latency, std dev for latency ((server_id, server_id) -> (probability of message being dropped, mean latency, standard deviation)
    pub(crate) fn new(
        network_connections: HashMap<
            (ServerId, ServerId),
            (PacketLossProbability, LatencyMean, LatencyStdDev),
        >,
    ) -> Self {
        let server_connections = network_connections
            .keys()
            .into_iter()
            .cloned()
            .collect::<HashSet<_>>();
        let network: HashMap<(ServerId, ServerId), NetworkConnectionQuality> = network_connections
            .into_iter()
            .map(|((from, to), (drop_probability, mean_latency, std_dev))| {
                assert!(
                    drop_probability.0 >= 0.0 && drop_probability.0 <= 1.0,
                    "(from={from:?}, to={to:?}): Drop probability should be between 0 and 1",
                    from=from,
                    to=to,
                );
                assert!(
                    mean_latency.0 >= 0.0,
                    "(from={from:?}, to={to:?}): Latency should be greater than or equal to 0",
                    from=from,
                    to=to,
                );
                assert!(
                    std_dev.0 >= 0.0,
                    "(from={from:?}, to={to:?}): Standard deviation should be greater than or equal to 0",
                    from=from,
                    to=to,
                );
                assert!(
                    server_connections.contains(&(to, from)),
                    "Connection (from={from:?}, to={to:?}) should be symmetric, i.e. (from={to:?}, to={from:?}) should also be present"
                );
                ((from, to), NetworkConnectionQuality {
                    packet_loss: Bernoulli::new(drop_probability.0)
                        .expect("SIM: Could not create Bernoulli distribution for packet loss"),
                    latency: LogNormal::new(mean_latency.0.ln(), std_dev.0)
                        .expect("SIM: Could not create LogNormal distribution for latency")
                })
            }).collect();

        let (outbound_message_tx, outbound_message_rx) = mpsc::channel();
        let (timer_tx, timer_rx) = mpsc::channel();

        let server_ids: HashSet<ServerId> = network.keys().map(|(from, _)| from).cloned().collect();
        let servers = HashMap::new();

        SimNetwork {
            server_ids,
            servers,
            connections: network,
            outbound_message_tx,
            outbound_message_rx,
            timer_tx,
            maybe_timer_rx: Some(timer_rx),
        }
    }

    /// Creates a network with the same packet loss and latency for all connections
    pub(crate) fn with_defaults(
        num_servers: u64,
        packet_loss: PacketLossProbability,
        mean_latency: LatencyMean,
        latency_std_dev: LatencyStdDev,
    ) -> Self {
        let mut network = HashMap::new();
        for from in 0..num_servers {
            for to in 0..num_servers {
                if from != to {
                    network.insert(
                        (ServerId(from), ServerId(to)),
                        (
                            packet_loss.clone(),
                            mean_latency.clone(),
                            latency_std_dev.clone(),
                        ),
                    );
                }
            }
        }
        SimNetwork::new(network)
    }

    /// Called by the simulator when it is creating server processes
    /// After the network has been initialized it uses this method
    /// to take ownership of the transport object and give it to the server process
    pub(crate) fn join_network_and_take_transport_connector(
        &mut self,
        server_id: ServerId,
    ) -> SimNetworkRaftTransportConnector {
        let (inbound_message_tx, inbound_message_rx) = mpsc::channel();
        self.servers.insert(
            server_id,
            NetworkNode {
                incoming_message_tx: inbound_message_tx,
            },
        );
        SimNetworkRaftTransportConnector::new(
            self.outbound_message_tx.clone(),
            inbound_message_rx,
            self.timer_tx.clone(),
        )
    }

    pub(crate) fn take_timer_rx(&mut self) -> mpsc::Receiver<WakeUpAtOrBefore> {
        self.maybe_timer_rx
            .take()
            .expect("SIM: Timer already taken!")
    }

    /// Used by tests to partition the network into multiple partitions, where each partition is a disjoin set of server IDs
    /// Servers in each partition are connected to each other, but servers in different partitions are not connected
    pub(crate) fn partition_network(&mut self, partitions: Vec<HashSet<ServerId>>) {
        // Validate that sets are disjoint, i.e. no server is in multiple partitions
        let mut all_servers = HashSet::new();
        for partition in &partitions {
            for server in partition {
                assert!(
                    !all_servers.contains(server),
                    "Server {server:?} is in multiple partitions",
                    server = server
                );
                all_servers.insert(server);
            }
        }
        // Validate that all servers in the network are in a partition
        for (from, to) in self.connections.keys() {
            assert!(
                all_servers.contains(from) && all_servers.contains(to),
                "Server {from:?} or server {to:?} is not in any partition",
                from = from,
                to = to
            );
        }
        // Set packet loss to 1.0 for all connections between servers in different partitions
        let keys: Vec<(ServerId, ServerId)> =
            self.connections.keys().into_iter().cloned().collect();
        for (from, to) in keys {
            let from_partition = partitions
                .iter()
                .find(|partition| partition.contains(&from))
                .unwrap();
            if !from_partition.contains(&to) {
                self.connections.get_mut(&(from, to)).unwrap().packet_loss =
                    Bernoulli::new(1.0).unwrap();
            }
        }
    }

    pub(crate) fn heal_network_partition(&mut self) {
        for connection in self.connections.values_mut() {
            connection.packet_loss = Bernoulli::new(0.01).unwrap();
        }
    }

    /// Can be used by tests to change the probability of messages being dropped between two servers
    pub(crate) fn update_connection_packet_loss(
        &mut self,
        from: ServerId,
        to: ServerId,
        packet_loss: PacketLossProbability,
    ) {
        assert!(
            packet_loss.0 >= 0.0 && packet_loss.0 <= 1.0,
            "(from={from:?}, to={to:?}): Packet loss probability should be between 0 and 1",
            from = from,
            to = to,
        );
        let connection = self.connections.get_mut(&(from, to)).expect(&format!(
            "SIM: Should have a connection between server {from:?} and server {to:?}",
            from = from,
            to = to
        ));
        connection.packet_loss = Bernoulli::new(packet_loss.0).unwrap();
    }

    /// Can be used by tests to change the latency profile of messages sent from one server to another
    pub(crate) fn update_connection_latency(
        &mut self,
        from: ServerId,
        to: ServerId,
        mean_latency: LatencyMean,
        latency_std_dev: LatencyStdDev,
    ) {
        assert!(
            mean_latency.0 >= 0.0,
            "SIM: (from={from:?}, to={to:?}): Latency should be greater than or equal to 0",
            from = from,
            to = to,
        );
        assert!(
            latency_std_dev.0 >= 0.0,
            "SIM: (from={from:?}, to={to:?}): Standard deviation should be greater than or equal to 0",
            from = from,
            to = to,
        );
        let connection = self.connections.get_mut(&(from, to)).expect(&format!(
            "SIM: Should have a connection between server {from:?} and server {to:?}",
            from = from,
            to = to
        ));
        connection.latency = LogNormal::new(mean_latency.0.ln(), latency_std_dev.0).unwrap();
    }

    /// Looks at the what server the message is from and what server it should be delivered to and uses
    /// the network configuration to determine when and if a message should be delivered and with what latency
    /// This is called by the simulator
    fn determine_when_and_if_message_should_be_delivered(
        &self,
        message: RpcMessage<SimLogCommand>,
        rng: &mut ChaCha8Rng,
    ) -> Option<(RpcMessage<SimLogCommand>, SimTime)> {
        let to = message.to();
        let from = message.from();

        let time = MockClock::time();

        let connection = self.connections.get(&(from, to)).expect(&format!(
            "Should have a connection between server {from:?} and server {to:?}",
            from = from,
            to = to
        ));
        let drop_message = connection.packet_loss.sample(rng);
        let message_latency = connection
            .latency
            .sample(rng)
            .to_u64()
            .expect("SIM: Could not convert latency to u64");
        let message_time = time + Duration::from_millis(message_latency);
        if drop_message {
            trace!(
                "DROPPING NETWORK MESSAGE: from {from:?} to {to:?} at {time:?}ms - {message:?}",
                from = from,
                to = to,
                time = time.as_millis(),
                message = message
            );
            None
        } else {
            trace!(
                "QUEUEING NETWORK MESSAGE: from {from:?} to {to:?} at {message_time:?}ms with latency {message_latency:?} - {message:?}",
                from = from,
                to = to,
                message_time = message_time.as_millis(),
                message_latency = message_latency,
                message = message
            );
            Some((message, SimTime(message_time)))
        }
    }

    /// This is called by the simulator to get all messages that have been sent from server processes
    /// to the network that have not been queued in the simulator yet
    pub(crate) fn get_all_queued_outbound_messages(
        &mut self,
        rng: &mut ChaCha8Rng,
        log: &mut SimLog,
    ) -> Vec<(RpcMessage<SimLogCommand>, SimTime)> {
        let mut messages: Vec<(RpcMessage<SimLogCommand>, SimTime)> = Vec::new();

        while let Ok(message) = self.outbound_message_rx.try_recv() {
            let message_cloned = message.clone();
            if let Some(message_to_be_delivered) =
                self.determine_when_and_if_message_should_be_delivered(message, rng)
            {
                messages.push(message_to_be_delivered);
            } else {
                let time = MockClock::time();
                log.push(SimLogEntry::EventProcessed(
                    SimTime(time),
                    LoggedSimEvent::DroppedNetworkMessage(SimTime(time), message_cloned),
                ));
            }
        }

        messages
    }

    /// Called by the simulator to actually deliver the message to the server process once it is time to deliver it
    pub(crate) fn deliver_message(&mut self, target: ServerId, message: RpcMessage<SimLogCommand>) {
        let network_node = self.servers.get_mut(&target).expect(&format!(
            "Should have a server with ID {to:?} in the simulation",
            to = target
        ));

        if let Err(_) = network_node.incoming_message_tx.send(message) {
            debug!("SIM: Could not send network message to server (raft thread shutdown?)");
        }
    }
}

mod tests {
    use std::time::Duration;

    use raft_consensus::rpc_messages::RpcMessage;
    use raft_consensus::{
        rpc_messages::Request, rpc_messages::RequestVote, LogIndex, RaftTransportConnector,
        RaftTransportError, ServerId, TermIndex,
    };
    use rand::RngCore;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use tracing::info;
    use uuid::Uuid;

    use crate::simulator::sim_log::SimLog;

    use super::{LatencyMean, LatencyStdDev, PacketLossProbability, SimNetwork};

    fn new_rng(maybe_seed: Option<u64>) -> ChaCha8Rng {
        match maybe_seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => {
                let mut rng = ChaCha8Rng::from_entropy();
                let seed = rng.next_u64();
                info!("====================================");
                info!("RNG SEED FOR TESTS: {seed}", seed = seed);
                info!("====================================");
                ChaCha8Rng::seed_from_u64(seed)
            }
        }
    }

    fn new_sim_log(log_file_name: Option<&str>) -> SimLog {
        let log_file_path = log_file_name.map(|filename| {
            let pwd = std::env::current_dir().unwrap();
            pwd.join("..").join(filename)
        });
        SimLog::new(log_file_path)
    }

    #[test]
    fn it_should_return_a_vec_with_all_queued_outbound_messages_from_servers() {
        let mut rng = new_rng(None);

        let mut network = SimNetwork::with_defaults(
            2,
            PacketLossProbability(0.0),
            LatencyMean(0.0),
            LatencyStdDev(0.0),
        );

        let mut originating_server_transport =
            network.join_network_and_take_transport_connector(ServerId(0));

        let outgoing_message = Request::RequestVote(RequestVote {
            request_id: Uuid::new_v4(),
            from: ServerId(0),
            to: ServerId(1),
            term: TermIndex(1),
            last_log_index: LogIndex(0),
            last_log_term: TermIndex(0),
        });
        let expected_message = outgoing_message.clone();

        if let Err(_) = originating_server_transport.enqueue_outgoing_request(outgoing_message) {
            panic!("Could not enqueue outgoing request! (transport shutdown)");
        }

        let mut sim_log = new_sim_log(None);
        let messages = network.get_all_queued_outbound_messages(&mut rng, &mut sim_log);
        assert_eq!(messages.len(), 1);

        let (message, _) = messages.get(0).unwrap();
        match message {
            RpcMessage::Request(request) => {
                assert_eq!(request, &expected_message);
            }
            _ => panic!("Expected a request from node"),
        }
    }

    #[test]
    fn it_should_drop_messages_if_servers_are_in_different_network_partitions() {
        let mut rng = new_rng(None);

        let mut network = SimNetwork::with_defaults(
            2,
            PacketLossProbability(1.0),
            LatencyMean(0.0),
            LatencyStdDev(0.0),
        );

        let mut originating_server_transport =
            network.join_network_and_take_transport_connector(ServerId(0));

        let outgoing_message = Request::RequestVote(RequestVote {
            request_id: Uuid::new_v4(),
            from: ServerId(0),
            to: ServerId(1),
            term: TermIndex(1),
            last_log_index: LogIndex(0),
            last_log_term: TermIndex(0),
        });

        if let Err(_) = originating_server_transport.enqueue_outgoing_request(outgoing_message) {
            panic!("Could not enqueue outgoing request! (transport shutdown)");
        }

        let mut sim_log = new_sim_log(None);
        let messages = network.get_all_queued_outbound_messages(&mut rng, &mut sim_log);
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn it_should_deliver_message_to_transport_for_server() {
        let mut network = SimNetwork::with_defaults(
            2,
            PacketLossProbability(0.0),
            LatencyMean(0.0),
            LatencyStdDev(0.0),
        );

        let mut dest_server_transport =
            network.join_network_and_take_transport_connector(ServerId(0));

        let incoming_message = Request::RequestVote(RequestVote {
            request_id: Uuid::new_v4(),
            from: ServerId(1),
            to: ServerId(0),
            term: TermIndex(1),
            last_log_index: LogIndex(0),
            last_log_term: TermIndex(0),
        });
        let expected_message = incoming_message.clone();

        let dest_server_thread = std::thread::spawn(move || {
            match dest_server_transport.wait_for_next_incoming_message(Duration::from_secs(1)) {
                Ok(Some(message)) => message,
                Ok(None) => panic!("Max wait reached"),
                Err(RaftTransportError::TransportShutdown) => panic!("Transport shutdown"),
            }
        });

        network.deliver_message(ServerId(0), RpcMessage::Request(incoming_message));

        let message = dest_server_thread.join().unwrap();
        match message {
            RpcMessage::Request(request) => {
                assert_eq!(expected_message, request);
            }
            _ => panic!("Expected a request from node"),
        }
    }
}
