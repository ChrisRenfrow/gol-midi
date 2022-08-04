use gol_midi::config::{Args, Config};

use clap::Parser;
use std::{env, process};

fn main() {
    let args = Args::parse();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = gol_midi::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
