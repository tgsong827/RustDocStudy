extern crate greprs;

use std::{env, process};

use greprs::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In File {}", config.filename);

    if let Err(e) = greprs::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}

