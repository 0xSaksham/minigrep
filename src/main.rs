use minigrep::Config;
use std::{env, process};

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguements: {err}");
        process::exit(1);
    });

    println!(
        "Searching for word:{} , in file : {}.",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
