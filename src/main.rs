use colored::Colorize;
use minigrep::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{} Problem parsing arguments: {}", "[ ERROR ]".red(), err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("{} Application error: {}", "[ ERROR ]".red(), e);
        process::exit(1);
    };
}
