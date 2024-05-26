use clap::Parser;
use pong::{cli::Cli, networking::network};

fn main() {
    let cli = Cli::parse();
    network(&cli.mode, &cli.host, &cli.port);
}
