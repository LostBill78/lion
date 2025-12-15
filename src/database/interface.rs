use std::{fs::{File, OpenOptions}, io, os::fd::AsRawFd, panic::{set_hook, take_hook}};

use anyhow::{Result, bail};

use crate::terminal::{Position, Terminal};

use super::{buffers, commands::{mega_command::MegaCommand,
            sql_command::SqlCommandResult}};

use super::ConnectDb;


pub struct Pager {
    file_name: String,
    file_descriptor: i32,
    mega_command: MegaCommand,
    should_quit: bool,
    connect: ConnectDb,
}

impl Default for Pager {
    fn default() -> Self {
        Self { 
            file_name: Default::default(), 
            file_descriptor: Default::default(),
            should_quit: false,
            mega_command: Default::default(),
            connect: Default::default(),
        }
    }
}

impl Pager {
    pub fn new() -> Result<Self> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        // Find and open the DB file
        let connect = ConnectDb::connect_database()?;
        // create an instance of the interface 
        let mut pager = Self::default();
        pager.connect = connect;
        pager.file_name = pager.connect.file_name.clone();
        Terminal::initialize()?;
        // Handle the connection to the database
        let mut file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(&pager.file_name)?;
        pager.file_descriptor = file.as_raw_fd();

        Ok(pager)
    }

    pub fn control(&mut self) -> Result<()> {
        if self.db_ok() == false {
            bail!("Bad Database!");
        }
        Terminal::move_cursor_to(Position {col: 0, row: 0})?;
        loop {
            if self.should_quit {
                break;
            }
            let _ = Terminal::print_prompt();
            let input_buffer = Self::read_input();
            if let Ok(input_value) = &input_buffer {
                if input_value.buffer.len() > 2 {
                    if input_value.buffer[0] == b'.' {
                        match MegaCommand::do_mega_command(&input_value) {
                            Ok(MegaCommand::Exit) => { self.should_quit = true; },
                            Ok(MegaCommand::Success) => (),
                            Ok(MegaCommand::UnknownCommand) => {
                                Terminal::print(format!("Unknown command entered: {:?}\n", String::from_utf8(input_value.buffer.clone())))?;
                            },
                            Err(_) => (),
                        }
                    } else {
                        match SqlCommandResult::initiate_conversion(input_value) {
                            Ok(command) => { SqlCommandResult::execute_command(&command); },
                            Err(e) => {
                                let _ = Terminal::print(format!("An error has occurred: **{}\n", e));
                                let _ = Terminal::print(format!("Command Entered: {:?}\n", String::from_utf8(input_value.buffer.clone()).unwrap()));
                                continue;
                            },
                        }
                    }
                }
            }
            if let Err(e) = &input_buffer {

            }
        }
        Ok(())
    }

    fn read_input() -> Result<buffers::InputBuffer> {
        let mut input: String = String::new();
        let stdin = io::stdin();

        let bytes_read = stdin.read_line(&mut input).unwrap();

        Ok(buffers::InputBuffer { 
            buffer: input.trim_end_matches('\n').into(), 
            buffer_length: bytes_read as u32,
            input_length:  bytes_read.saturating_sub(1) as u32,
        })
        
    }

    fn db_ok(&mut self) -> bool {
        return true;
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        if self.should_quit {
            self.connect.close_database().unwrap();
            Terminal::terminate().unwrap();
            Terminal::print("Goodbye.\n".to_owned()).unwrap();
        }
    }
}