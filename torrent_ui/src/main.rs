use clap::{Parser, Subcommand};
use torrent_ui::Magnet;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.cmd {
        SubCmd::Parse { magnet } => {
            dbg!(magnet);
        }
    }
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
