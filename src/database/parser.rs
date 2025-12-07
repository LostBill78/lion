
use anyhow::*;
use super::sql_command::Command;
use super::buffers::InputBuffer;
use super::lexer::Lexer;

#[derive(Debug)]
pub struct Dictionary {
    pub command: super::lexer::Token,
    // pub table_name: String,
    // pub columns: Vec<String>,
    // pub values: Vec<String>,        // convert eveything to Strings in the database
}


impl Dictionary {
    pub fn input_parser(input_buffer: &InputBuffer) -> Result<Dictionary> {
        let lexer_result = Lexer::tokenizer(input_buffer);

        // let mut command: Command = Command::select;
        // if input_buffer.buffer.starts_with(b"insert")  {
        //     command = Command::insert;
        // }

        
        Ok(Dictionary {
            command: lexer_result[0].clone(),
            // table_name: (),
            // columns: (),
            // values: (),
        })
    }
}
