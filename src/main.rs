#![allow(warnings)]
#[allow(dead_code)]

mod database;
mod terminal;
use std::{env, process::exit};

use database::interface::Pager;

use crate::terminal::Terminal;


fn main() {
    // Connect to the interface
    let mut pager = Pager::new().unwrap();
    match pager.control() {
        Ok(_) => (),
        Err(e) => { 
            Terminal::print(format!("An error occured: {}\n", e));
        },
    }
}