//! minigrep: print the lines of a file that contain a query.
//!
//! ```text
//! cargo run -p minigrep -- frog projects/minigrep/poem.txt
//! IGNORE_CASE=1 cargo run -p minigrep -- FROG projects/minigrep/poem.txt
//! ```
//!
//! This file is complete. The functions it calls live in `lib.rs`.

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }
}
