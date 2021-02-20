use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};

use crate::process_stats::ProcessStats;

const DEFAULT_ADDR: &str = "127.0.0.1:0";

let listener: String

pub async fn listen() -> Result<bool> {
    // TODO: Take mutex so that we cannot call only one.

    let listener = TcpListener::bind(DEFAULT_ADDR).await?;
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            handle(socket).await;
        });
    }
    //Ok(true)
}

async fn handle(socket: TcpStream) {
    let process_stats = ProcessStats::get()
        .await
        .expect("could not get stats for running process");
    println!("{:?}", process_stats);
}
