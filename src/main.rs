#![allow(unused_variables, unused_imports, dead_code, unused_must_use)]

extern crate grepit;
use std::{env, process, io::{Read, prelude}, error::Error, fs::File};
use grepit::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    println!("Searching for term: '{}'", config.query);
    println!("In file: '{}'", config.filename);
    if let Err(e) = grepit::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

