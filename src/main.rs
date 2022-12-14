use std::env;
use std::process;

use minigrep::Config;

// To run with envs you will need to like
// IGNORE_CASE=1 cargo run to poem.txt

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(2);
    }
}
