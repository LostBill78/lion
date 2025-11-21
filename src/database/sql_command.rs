use anyhow::Result;
use super::buffers::InputBuffer;

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

}
#[derive(Debug)]
pub struct Dictionary {
    command: Command,
    table_name: String,
    columes: Vec<String>,
    values: Vec<String>,        // convert eveything to Strings in the database
}

#[derive(Debug)]
pub enum SqlResponse {
    Success,
    Table_full,
    Database_full,
}

pub struct SqlCommandResult {
    response: SqlResponse,
    output: Vec<u8>,
}

impl SqlCommandResult {
    pub fn initiate_conversion(&mut self, input_buffer: InputBuffer) -> Result<()> {
        // Send input to the lexer for breakdown
        let lexer_result = self.input_lexer(input_buffer)?;
        match lexer_result.command {
            Command::insert => todo!(),
            Command::select => todo!(),
            Command::create => todo!(),
            Command::join => todo!(),
        }
        Ok(())
    }
    

    fn input_lexer(&mut self, input_buffer: InputBuffer) -> Result<Dictionary> {

        Ok(Dictionary {
            command: todo!(),
            table_name: todo!(),
            columes: todo!(),
            values: todo!(),
        })
    }
}
