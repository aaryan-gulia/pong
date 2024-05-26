use clap::Parser;
use pong::cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}
