#![allow(warnings)]
#[allow(dead_code)]

mod database;
mod terminal;
use std::{env, process::exit};

use database::interface::Pager;


fn main() {
    // We must require a file name to be entered
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must enter the database name!\n");
        exit(1);
    }
    // Next connect to the interface
    Pager::new(&args[1]).unwrap().control().unwrap();
}