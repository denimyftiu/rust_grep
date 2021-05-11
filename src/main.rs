//! # Example usage.
//!
//! ```
//! rust_grep dep Cargo.toml
//! ```
use std::env;
use std::process;

use rust_grep::Config;

fn main() {
    // Get the config from user input.
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    // Run the search based on config.
    if let Err(err) = rust_grep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    };
}
