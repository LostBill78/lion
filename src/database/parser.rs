
use anyhow::*;
use crate::database::lexer::Token;
use crate::terminal::Terminal;

use super::sql_command::Command;
use super::buffers::InputBuffer;
// use super::lexer::Lexer;
use super::lexer::*;

#[derive(Debug, Default)]
pub struct Dictionary {
    pub command: Token,
    // pub table_name: String,
    // pub columns: Vec<String>,
    // pub values: Vec<String>,        // convert eveything to Strings in the database
}


impl Dictionary {
    pub fn new_parser() -> Self {
        Self::default()
    }
    pub fn input_parser(&mut self, input_buffer: &InputBuffer) -> Result<Dictionary> {
        let mut lexer_result = Lexer::tokenizer(input_buffer);

        // Need to complete some syntax checking
        let be = self.do_syntax_check(&lexer_result)?;

        Ok(Dictionary {
            command: lexer_result[0].clone(),
            // table_name: (),
            // columns: (),
            // values: (),
        })
    }

    fn do_syntax_check(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        let command = tokens[0].clone();
        match command {
            Token::Insert => {
                self.syntax_check_insert(&tokens)?;
            },
            Token::Select => {
                self.syntax_check_select(&tokens)?;
            },
            Token::Create => {
                self.syntax_check_create(&tokens)?;
            },
            _ => (),
        }

        Ok(Dictionary {
            command: command,

        })
    }
    fn syntax_check_insert(&self, tokens: &Vec<Token>) -> Result<()> {
        

        for index in 0..tokens.len() {
            let my_token = &tokens[index];
            if (index == 1 && *my_token != Token::Into) {
                bail!("Syntax err on entry!");
            }
            if index == 2 {
                // Test valid table name
                if let Token::Chars(e) = my_token {
                    let _ = Terminal::print(format!("look for table = {}\n", e));
                }
            }
        }
        Ok(())
    }
    fn syntax_check_select(&self, tokens: &Vec<Token>) -> Result<()> {

        Ok(())
    }
    fn syntax_check_create(&self, tokens: &Vec<Token>) -> Result<()> {
        for index in 0..tokens.len() {
            let my_token = &tokens[index];
            if (index == 1 && 
                (*my_token != Token::Table && 
                    *my_token != Token::Database)) {
                bail!("Syntax err on entry!");
            }
        }
        Ok(())
    }
}
