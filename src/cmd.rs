use anyhow::{anyhow, Result};

use crate::rprocess;

pub const CMD_MEM_STATS: &str = "memstats";

pub fn print_mem_stats() -> Result<bool> {
    println!("mem");
    Ok(true)
}

pub fn print_rust_processes() -> Result<bool> {
    let processes = rprocess::find_all().unwrap();
    for p in processes.iter() {
        println!("{}", p);
    }
    Ok(true)
}
