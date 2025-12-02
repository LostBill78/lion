
use anyhow::*;
use super::sql_command::Command;
use super::buffers::InputBuffer;


#[derive(Debug)]
pub struct Dictionary {
    pub command: Command,
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<String>,        // convert eveything to Strings in the database
}


impl Dictionary {
    pub fn input_parser(input_buffer: &InputBuffer) -> Result<Dictionary> {

        
        Ok(Dictionary {
            command: todo!(),
            table_name: todo!(),
            columns: todo!(),
            values: todo!(),
        })
    }
}
