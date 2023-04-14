mod app;

use std::{collections::HashMap, net::SocketAddr, time::Duration};

use crate::app::SingleValueStoreImpl;
use raft_consensus::{start_raft_in_new_thread, NoOpRaftEventCollector, RaftConfig, ServerId};
use raft_grpc::grpc_transport::RaftGrpcTransport;
use raft_grpc::proto::raft_consensus_server::RaftConsensusServer;
use single_value_store_proto::single_value_store::single_value_store_server::SingleValueStoreServer;
use tokio::select;
use tonic::transport::Server;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ID of this server
    #[arg(short, long)]
    server_id: u32,

    /// Port to listen on
    #[arg(short, long)]
    port: u16,

    /// Arg that contains the members of the cluster
    /// Command delimited list of server IDs/addresses
    /// Ex:
    /// 1,127.0.0.1:123,2,127.0.0.1:234,3,127.0.0.1:345
    #[arg(short, long)]
    cluster_members: String,

    /// Path to directory to store write ahead logs
    #[arg(short, long)]
    wal_log_dir: String,

    /// Leader heartbeat interval in milliseconds
    #[arg(short, long)]
    leader_heartbeat_ms: u64,
}

fn parse_cluster_members(cluster_members: &str) -> HashMap<ServerId, SocketAddr> {
    let mut cluster_members = cluster_members.split(',');
    let mut cluster = HashMap::new();
    while let Some(id) = cluster_members.next() {
        let id: u64 = id.parse().unwrap();
        let addr = cluster_members
            .next()
            .unwrap()
            .to_string()
            .parse()
            .expect("SERVER INIT: Could not parse server address");
        cluster.insert(ServerId(id), addr);
    }
    cluster
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Args::parse();

    let addr = SocketAddr::new(
        "0.0.0.0"
            .parse()
            .expect("SERVER INIT: Could not parse server IP"),
        args.port,
    );
    let server_id = ServerId(args.server_id.into());

    let server_id_to_addr = parse_cluster_members(&args.cluster_members);
    let other_servers = server_id_to_addr
        .iter()
        .filter(|(id, _)| **id != server_id)
        .map(|(id, _)| *id)
        .collect();

    let mut raft_grpc_transport =
        RaftGrpcTransport::start_grpc_transport(server_id.clone(), server_id_to_addr).await;
    let config = RaftConfig {
        leader_heartbeat_interval: Duration::from_millis(args.leader_heartbeat_ms),
        min_election_timeout_ms: 150,
        max_election_timeout_ms: 300,
    };
    let rng = ChaCha8Rng::from_entropy();
    let event_collector = NoOpRaftEventCollector {};
    let raft_thread = start_raft_in_new_thread(
        server_id.clone(),
        other_servers,
        args.wal_log_dir,
        config,
        rng,
        raft_grpc_transport.transport_bridge,
        event_collector,
    );
    raft_grpc_transport
        .grpc_server
        .register_raft_thread(raft_thread);

    let app = SingleValueStoreImpl {};

    select! {
        _ = raft_grpc_transport.message_sender_task => {},
        _ = Server::builder()
            .add_service(RaftConsensusServer::new(raft_grpc_transport.grpc_server))
            .add_service(SingleValueStoreServer::new(app))
            .serve(addr) => {},
    }

    Ok(())
}
