use std::env;
use std::process;

use rust_grep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(err) = rust_grep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    };
}
