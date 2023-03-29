#[macro_use]
extern crate steward;
#[macro_use]
extern crate lazy_static;

mod loc;

use loc::Loc;
use steward::{Env, Process, ProcessPool};

#[tokio::main]
async fn main() -> steward::Result<()> {
    let num_raft_nodes = 5;

    let server_tags = vec![
        "server-node(0)",
        "server-node(1)",
        "server-node(2)",
        "server-node(3)",
        "server-node(4)",
    ];

    let processes = (0..4)
        .map(|i| {
            raft_node(
                i.try_into().unwrap(),
                server_tags.get(i).unwrap(),
                num_raft_nodes,
                5000,
            )
        })
        .collect();

    ProcessPool::run(processes).await?;

    Ok(())
}

fn cluster_members_arg(num_servers: u16, starting_port: u16) -> String {
    let mut cluster_members = String::new();
    for i in 0..num_servers {
        if i > 0 {
            cluster_members.push_str(",");
        }
        cluster_members.push_str(&format!("{},127.0.0.1:{}", i, starting_port + i));
    }
    cluster_members
}

fn raft_node(
    server_id: u16,
    server_tag: &'static str,
    num_servers: u16,
    starting_port: u16,
) -> Process<Loc> {
    let cluster_members = cluster_members_arg(num_servers, starting_port);
    let root_loc = Loc::root();
    let wal_root_loc = root_loc
        .as_ref()
        .join(format!("target/wal-logs/{}", server_id));

    let wal_root_loc_cloned = wal_root_loc.clone();
    let wal_path = wal_root_loc_cloned.to_str().unwrap();

    std::fs::create_dir_all(wal_root_loc).expect(&format!(
        "Could not create wal log dir for server ID {:?}",
        server_id
    ));

    process! {
      tag: server_tag,
      cmd: cmd! {
        exe: format!(
            "cargo run --bin single_value_store -- --server-id {server_id} --cluster-members {cluster_members} --port {port} --wal-log-dir {wal_log_dir} --leader-heartbeat-ms 50",
            server_id = server_id,
            cluster_members = cluster_members,
            port = starting_port + server_id,
            wal_log_dir = wal_path),
        env: Env::empty(),
        pwd: root_loc,
        msg: "Running a reloadable server",
      },
    }
}
