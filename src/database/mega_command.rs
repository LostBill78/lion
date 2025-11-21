use crate::database::buffers::InputBuffer;
use anyhow::Result;


#[derive(Default)]
pub enum MegaCommand {
    Success,
    Exit,
    #[default]
    UnknownCommand,
}

impl MegaCommand{
    pub fn do_mega_command(input_buffer: &InputBuffer) -> Result<MegaCommand> {
        let mut result = MegaCommand::UnknownCommand;
        if input_buffer.buffer.starts_with(b".exit") {
            result = MegaCommand::Exit;
        }
        Ok(result)
    }
}