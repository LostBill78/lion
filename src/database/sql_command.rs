use anyhow::Result;
use crate::terminal::Terminal;

use super::buffers::InputBuffer;
use super::parser::Dictionary;

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    LParen,
    RParen,
    Comma,
    SemiColon,
}

#[derive(Debug)]
pub enum Command {
    insert,
    select,
    create,
    join,
    unknown,
}

#[derive(Debug)]
pub enum SqlResponse {
    Success,
    Table_full,
    Database_full,
    UnknownInput,
}

pub struct SqlCommandResult {
    response: SqlResponse,
    output: Vec<u8>,
}

impl SqlCommandResult {
    pub fn initiate_conversion(input_buffer: &InputBuffer) -> Result<()> {
        // Send input to the lexer for breakdown

        //? should this be sent to the parser first and let parse handle each item in turn
        let parser_result = Dictionary::input_parser(&input_buffer)?;
        match parser_result.command {
            Command::insert => {
                let _ = Terminal::print_line("Found command: (\"insert\")\n".to_owned());
            },
            Command::select => (),
            Command::create => (),
            Command::join => (),
            Command::unknown => todo!(),
        }
        Ok(())
    }
    

}
