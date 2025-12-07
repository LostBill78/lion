
use std::iter::{self, from_fn};

use anyhow::Result;
use crate::database::sql_command::Tokens;

use super::sql_command::Command;
use super::buffers::InputBuffer;


const INSERT: &str = "insert";
const SELECT: &str = "select";

#[derive(Debug, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Comma,
    And,
    Or,
    Insert,
    Select,
    Create,
    Into,
    Where,
    Values,
    Equal,
    Chars(String),
}

pub struct Lexer {

}

impl Lexer {
    
    pub fn tokenizer(input_buffer: &InputBuffer) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut input: String = String::from_utf8(input_buffer.buffer.clone()).unwrap();

        let mut iter = input.chars().peekable();

        while let Some(ch) = iter.next() {
            match ch {
                ch if ch.is_whitespace() => continue,
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                ',' => tokens.push(Token::Comma),

                'a'..'z' | 'A'..'Z' => {
                    let s: &str = &iter::once(ch)
                        .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_alphabetic())))
                        .collect::<String>();

                    match s {
                        INSERT => tokens.push(Token::Insert),
                        SELECT => tokens.push(Token::Select),

                        &_ => tokens.push(Token::Chars(s.to_string())),
                    }
                    // if s.to_lowercase() == "insert" {
                    //     tokens.push(Token::Insert);
                    // } else {
                    //     tokens.push(Token::Chars(s.to_string()));
                    // }
                },
                _ => {},
            }
        };

        tokens
    }
    // pub fn input_lexer(input_buffer: &InputBuffer) -> Result<Command>{
    //     let mut result = Command::unknown;
    //     if input_buffer.buffer.starts_with(b"insert") {
    //         result = Command::insert;
    //     }
    //     Ok(result)
    // }
}