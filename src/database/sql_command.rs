use anyhow::Result;
use crate::database::lexer::Token;
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
            Token::Insert => {
                let _ = Terminal::print_line(format!("found command: {}\n", "insert"));
            },
            Token::LeftParen => todo!(),
            Token::RightParen => todo!(),
            Token::Comma => todo!(),
            Token::And => todo!(),
            Token::Or => todo!(),
            Token::Select => {
                let _ = Terminal::print_line(format!("found command: {}\n", "select"));
            },
            Token::Create => todo!(),
            Token::Into => todo!(),
            Token::Where => todo!(),
            Token::Values => todo!(),
            Token::Equal => todo!(),
            Token::Chars(_) => todo!(),
        }
        Ok(())
    }
    

}
