use std::time::Duration;

use crate::cli::Mode;
use crate::client::*;
use crate::server::*;

pub fn network(mode: &Mode, host: &String, port: &String) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    match mode {
        Mode::Client => {
            println!("connect to: {}:{}", host, port);
            runtime.block_on(async {
                tokio::select! {
                    _ = client(host, port)=>{},
                    _ = tokio::signal::ctrl_c() => {}
                }
            });
            runtime.shutdown_timeout(Duration::from_secs(0));
        }
        Mode::Server => {
            println!("bind on: {}:{}", host, port);
            runtime.block_on(async {
                tokio::select! {
                    _ = server(host,port)=>{},
                    _ = tokio::signal::ctrl_c()=>{}
                }
            });
        }
    }
}
