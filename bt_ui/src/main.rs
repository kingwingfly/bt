use bt_lib::Magnet;
use clap::{Parser, Subcommand};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_env_filter("bt_lib=info,bt_ui=info")
        .without_time()
        .with_target(false)
        .init();
    if let Err(e) = run().await {
        error!(e);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.cmd {
        SubCmd::Parse { magnet } => {
            info!("{:#?}", magnet);
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCmd,
}

#[derive(Subcommand)]
enum SubCmd {
    /// Parse magnet link
    Parse { magnet: Magnet },
}
