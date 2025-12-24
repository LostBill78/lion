
use anyhow::*;
use crate::database::buffers::*;
use crate::database::tokenizer::lexer::Token;
use crate::terminals::Terminal;

//use super::super::buffers::InputBuffer;

use super::lexer::*;

#[derive(Debug, Default)]
pub struct Dictionary {
    pub command: Token,
    pub required: Token,
    pub object_name: String,
    pub columns: Vec<Column>,
    pub values: Vec<String>,        // convert eveything to Strings in the database
}


impl Dictionary {
    pub fn new_parser() -> Self {
        Self::default()
    }
    pub fn input_parser(&mut self, input_buffer: &InputBuffer) -> Result<Dictionary> {
        // Test the terminator on input string
        if input_buffer.buffer[input_buffer.buffer.len() -1] != b';' {
            bail!("Invalid input - Missing input terminator");
        }
        let mut lexer_result = Lexer::tokenizer(input_buffer);

        // Need to complete some syntax checking
        let be = self.do_syntax_check(&lexer_result)?;

        Ok(be)
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
            required: Token::Unknown,
            object_name: "Unknown".to_string(),
            columns: Vec::new(),
            values: Vec::new(),
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
        let mut object_name = String::new();
        if let Token::Chars(e) = my_table{
            object_name = e;
        }
        let mut columns = Vec::new();
        let mut values: Vec::<String> = Vec::new();
        Ok(Dictionary {
            command: tokens[0].clone(),
            required: tokens[1].clone(),
            object_name: object_name,
            columns: columns,
            values: values,
        })
    }
    fn build_update_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        Ok(Dictionary {
            command: tokens[0].clone(),
            required: Token::Unknown,
            object_name: "Unknown".to_string(),
            columns: Vec::new(),
            values: Vec::new(),
        })
    }
    fn build_select_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {

        Ok(Dictionary {
            command: tokens[0].clone(),
            required: Token::Unknown,
            object_name: "Unknown".to_string(),
            columns: Vec::new(),
            values: Vec::new(),
        })
    }
    fn build_create_dictionary(&mut self, tokens: &Vec<Token>) -> Result<Dictionary> {
        
        /* collect a name item */
        let mut object_name = String::new();
        let mut columns = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut required = Token::Unknown;
        let mut command = Token::Unknown;
        let mut capture_column: bool = false;
        let mut paren_count: i32 = 0;
        let mut column_posn: i32 = 0;
        let mut column: Column = Column::default();

        for mut index in 0..tokens.len() - 1 {
            Terminal::print(format!("Working index: {}\n", index));
            if index == 0 {
                command = tokens[index].clone();
            }
            if index == 1 {
                required = tokens[index].clone();
            }
            if index == 2 {
                let my_item = tokens[index].clone();
                if let Token::Chars(e) = my_item{
                    object_name = e;
                }
            }

            match tokens[index] {
                Token::LeftParen => {
                    paren_count += 1;
                    capture_column = true;
                },
                Token::RightParen => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        capture_column = false;
                    }
                },
                Token::Comma => {
                    columns.push(column);
                    // Reset column
                    column = Column::default();
                },
                _ => {
                    match column_posn {
                        0 => {
                            let my_table = tokens[index].clone();
                            if let Token::Chars(e) = my_table{
                                column.column_name = e;
                            }
                        },
                        1 => {
                            let my_table = tokens[index].clone();
                            if let Token::Chars(e) = my_table{
                                column.column_data_type = DataType::I64;
                            }
                        },
                        2 => {let my_table = tokens[index].clone();
                            if let Token::Chars(e) = my_table{
                                column.column_control = TypeDefine::default();
                            }
                        },
                        _ => (),
                    }
                    column_posn += 1;
                },
            }
        }

        Ok(Dictionary {
            command: tokens[0].clone(),
            required: tokens[1].clone(),
            object_name: object_name,
            columns: columns,
            values: values,
        })
    }

    fn capture_column_information(&self, tokens: &Vec<Token>, columns: &Vec<Column>, total_items: usize, current_index: &usize) -> usize {
        let start_index = *current_index;
        let mut result_index: usize = 0;
        let mut paran_count: usize = 0;
        for index in start_index..total_items {
            match tokens[index] {
                Token::LeftParen => paran_count += 1,
                Token::RightParen => paran_count -= 1,
                _ => (),
            }
            if paran_count == 0 {            Terminal::print(format!("Index: {}\n", index));

                result_index = index;
                break;
            }
        }
        result_index
    }
}
