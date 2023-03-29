use clap::{Parser, Subcommand};
use single_value_store_proto::single_value_store::single_value_store_client::SingleValueStoreClient;
use single_value_store_proto::single_value_store::{GetRequest, SetRequest};
use tonic::transport::Channel;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    server_address: String,
}

#[derive(Subcommand)]
enum Commands {
    Get,
    Set { value: u64 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    let channel = Channel::from_shared(format!("http://{}", cli.server_address))
        .expect("Failed to create channel")
        .connect()
        .await
        .expect("Failed to connect to server");

    let mut client = SingleValueStoreClient::new(channel.clone());

    info!("Connected to server: {:?}", cli.server_address);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Get => {
            info!("GET");
            let result = client.get(tonic::Request::new(GetRequest {})).await?;
            info!("Result: {:?}", result);
        }
        Commands::Set { value } => {
            info!("SET {}", value);
            let result = client
                .set(tonic::Request::new(SetRequest { value: *value }))
                .await?;
            info!("Result: {:?}", result);
        }
    }
    Ok(())
}
