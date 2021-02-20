use std::fmt;

use anyhow::{anyhow, Result};
use psutil::process;

/// A `Process` represents an OS process.
#[derive(Debug)]
pub struct Process {
    pid: u32,
    name: String,
    // Currently psutil supports ppid only for Linux.
    // TODO: Add macro to check target platform.
    //ppid: u32,
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.pid, self.name)
    }
}

/// Gives back all the Rust processes currently running on this host.
pub fn find_all() -> Result<Vec<Process>> {
    let ps = match process::processes() {
        Ok(p) => p,
        Err(e) => {
            return Err(anyhow!("failed to get processes: {}", e));
        }
    };

    extract_rust_process(ps)
}

/// Find Rust processes in the given processes.
fn extract_rust_process(ps: Vec<process::ProcessResult<process::Process>>) -> Result<Vec<Process>> {
    let mut res: Vec<Process> = Vec::with_capacity(ps.len());
    for p in ps {
        let p = match p {
            Ok(p) => p,
            Err(e) => continue,
        };
        /*let ppid = match p.ppid() {
            Ok(p) => match p {
                Some(p) => p,
                None => 0u32,
            },
            Err(e) => 0u32,
        };*/
        let name = match p.name() {
            Ok(n) => n,
            Err(e) => String::from(""),
        };
        res.push(Process {
            pid: p.pid(),
            name: name,
        })
    }
    Ok(res)
}
