use clap::Parser;

#[derive(Debug, Clone)]
pub enum Mode {
    Client,
    Server,
}

impl std::str::FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "client" {
            Ok(Mode::Client)
        } else if s.to_lowercase() == "server" {
            Ok(Mode::Server)
        } else {
            return Err(Self::Err::from("Wrong Command: Must be Server or Client"));
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Server or Client
    #[arg(long, short)]
    pub mode: Mode,

    /// host
    #[arg(long, default_value_t = String::from("localhost"))]
    pub host: String,

    /// port
    #[arg(long, default_value_t = String::from("2323"))]
    pub port: String,
}
