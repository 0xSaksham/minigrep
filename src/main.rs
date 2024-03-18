use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguements: {err}");
        process::exit(1);
    });

    println!(
        "Searching for word:{} , in file : {}.",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
