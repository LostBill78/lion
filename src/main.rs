#![allow(warnings)]
#[allow(dead_code)]

mod database;
mod terminal;
use std::{env, process::exit};

use database::interface::Pager;

use crate::terminal::Terminal;


fn main() {
    // We must require a file name to be entered
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must enter the database name!\n");
        exit(1);
    }
    // Next connect to the interface
    let mut pager = Pager::new(&args[1]).unwrap();
    match pager.control() {
        Ok(_) => (),
        Err(e) => { 
            Terminal::print(format!("An error occured: {}\n", e));
        },
    }
}