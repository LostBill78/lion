use std::{fs::{File, OpenOptions}, io, os::fd::AsRawFd, panic::{set_hook, take_hook}};

use anyhow::Result;

use crate::terminal::{Position, Terminal};

use super::{command, buffers, mega_command::MegaCommand};


pub struct Pager {
    file_name: String,
    file_descriptor: i32,
    mega_command: MegaCommand,
    should_quit: bool,
}

impl Default for Pager {
    fn default() -> Self {
        Self { 
            file_name: Default::default(), 
            file_descriptor: Default::default(),
            should_quit: false,
            mega_command: Default::default(),
        }
    }
}

impl Pager {
    pub fn new(file_name: &str) -> Result<Self> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        // create an instance of the interface 
        let mut pager = Self::default();
        pager.file_name = String::from(file_name);
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
        Terminal::move_cursor_to(Position {col: 0, row: 0})?;
        loop {
            if self.should_quit {
                break;
            }
            let input_buffer = Self::read_input();
            match input_buffer {
                Ok(InputBuffer) => {
                    if InputBuffer.buffer[0] == b'.' {
                        match MegaCommand::do_mega_command(&InputBuffer) {
                            Ok(MegaCommand::Exit) => { self.should_quit = true; },
                            Ok(MegaCommand::Success) => (),
                            Ok(MegaCommand::UnknownCommand) => {
                                Terminal::print(format!("Unknown command entered: {:?}\n", String::from_utf8(InputBuffer.buffer.clone())))?;
                            },
                            Err(_) => (),
                        }
                    } else {
                        Terminal::print(format!("Command entered: {:?}\n", String::from_utf8(InputBuffer.buffer.clone())))?;                        
                    }
                }
                Err(e) => {

                }
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
}

impl Drop for Pager {
    fn drop(&mut self) {
        if self.should_quit {
            Terminal::terminate().unwrap();
            Terminal::print("Goodbye.\n".to_owned()).unwrap();
        }
    }
}