use std::io::{Write, stdout};

use crossterm::{Command, cursor::MoveTo, queue, style::Print, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
const NAME: &str = env!("CARGO_PKG_NAME");
pub struct Position {
    pub col: usize,
    pub row: usize,
}


pub struct Terminal;

impl Terminal {
    pub fn terminate() -> anyhow::Result<()> {
        Self::leave_alternate_screen()?;
        Self::execute()?;
//        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> anyhow::Result<()> {
//        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::execute()?;
        Ok(())
    }
    pub fn enter_alternate_screen() -> anyhow::Result<()> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> anyhow::Result<()> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn print(text: String) -> anyhow::Result<()> {
        Self::queue_command(Print(text))?;
        Ok(())
    }
    pub fn print_line(text: String) -> anyhow::Result<()> {
        Self::print(text);
        Self::execute();
        Ok(())
    }
    pub fn print_prompt() -> anyhow::Result<()> {
        Self::print(format!("{} > ", NAME))?;
        Self::execute()?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> anyhow::Result<()> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }
    fn execute() ->anyhow::Result<()> {
        stdout().flush()?;
        Ok(())
    }
    fn queue_command<T: Command>(command: T) -> anyhow::Result<()> {
        queue!(stdout(), command)?;
        Self::execute()?;
        Ok(())
    }
}