use std::env;
use std::process;

use minigrep::Config;

fn main() {

    // unwrap_or_else says "if I get an Ok, get the value out of it and bind it to config,
    // otherwise, pass the error into the closure"
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
