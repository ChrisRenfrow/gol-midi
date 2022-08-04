use gol_midi::config::Config;

use clap::*;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = gol_midi::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
