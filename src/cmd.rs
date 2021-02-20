use anyhow::{anyhow, Result};

use crate::rprocess;

pub fn print_mem_stats() -> Result<bool> {
    println!("mem");
    Ok(true)
}

pub fn print_rust_processes() -> Result<bool> {
    let processes = rprocess::find_all();
    for p in processes.iter() {
        println!("{:?}", p);
    }
    Ok(true)
}
