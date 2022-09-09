use std::{env, process};
use merge_bilibili::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = merge_bilibili::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
