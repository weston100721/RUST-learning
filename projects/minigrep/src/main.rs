extern crate minigrep;

use minigrep::Config;
use minigrep::run;
use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::result::Result;
use std::result::Result::Err;
use std::result::Result::Ok;
use std::string::String;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

