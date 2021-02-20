mod app;
mod cmd;
mod rprocess;

use std::env;
use std::process;

use anyhow::Result;

fn run() -> Result<bool> {
    let matches = app::build_app().get_matches_from(env::args_os());
    if let Some(ref sub_m) = matches.subcommand_matches("memstats") {
        return cmd::print_mem_stats();
    }

    return cmd::print_rust_processes();
}

#[tokio::main]
async fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            eprintln!("{:#}", err);
            process::exit(1);
        }
    }
}
