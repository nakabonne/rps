use anyhow::{anyhow, Result};

use crate::process_stats::ProcessStats;

pub async fn listen() -> Result<bool> {
    // TODO: Take mutex so that we cannot call only one.

    let process_stats = ProcessStats::get()
        .await
        .expect("could not get stats for running process");
    println!("{:?}", process_stats);
    Ok(true)
}
