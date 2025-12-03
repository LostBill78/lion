
use anyhow::Result;
use super::sql_command::Command;
use super::buffers::InputBuffer;


pub struct Lexer {

}

impl Lexer {
    pub fn input_lexer(input_buffer: &InputBuffer) -> Result<Command>{
        let mut result = Command::unknown;
        if input_buffer.buffer.starts_with(b"insert") {
            result = Command::insert;
        }
        Ok(result)
    }
}