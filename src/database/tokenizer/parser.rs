
use anyhow::*;
use crate::database::tokenizer::lexer::Token;
use crate::terminal::Terminal;

use super::super::commands::sql_command::Command;
use super::super::buffers::InputBuffer;

use super::lexer::*;

#[derive(Debug, Default)]
pub struct Dictionary {
    pub command: Token,
    pub table_name: String,
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
            table_name: "Unknown".to_string(),
            // columns: (),
            // values: (),
        })
    }

    fn do_syntax_check(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        let command = tokens[0].clone();
        match command {
            Token::Insert => {
                self.syntax_check_insert(&tokens)?;
                return self.build_insert_dictionary(&tokens);
            },
            Token::Update => {
                self.syntax_check_update(&tokens)?;
                return self.build_insert_dictionary(&tokens);
            },
            Token::Select => {
                self.syntax_check_select(&tokens)?;
                return self.build_select_dictionary(&tokens);
            },
            Token::Create => {
                self.syntax_check_create(&tokens)?;
                return self.build_create_dictionary(&tokens);
            },
            _ => (),
        }

        Ok(Dictionary {
            command: command,
            table_name: "Unknown".to_string(),

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
    fn syntax_check_update(&self, tokens: &Vec<Token>) -> Result<()> {
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

    /*  Section Build Dictionary   */
    fn build_insert_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        let my_table = tokens[2].clone();
        let mut table_name = String::new();
        if let Token::Chars(e) = my_table{
            table_name = e;
        }

        Ok(Dictionary {
            command: tokens[0].clone(),
            table_name: table_name,
        })
    }
    fn build_update_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        Ok(Dictionary {
            command: tokens[0].clone(),
            table_name: "Unknown".to_string(),
        })
    }
    fn build_select_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        Ok(Dictionary {
            command: tokens[0].clone(),
            table_name: "Unknown".to_string(),
        })
    }
    fn build_create_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        Ok(Dictionary {
            command: tokens[0].clone(),
            table_name: "Unknown".to_string(),
        })
    }
}
