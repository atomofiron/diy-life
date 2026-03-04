use crate::types::TerminalDisplay;
use ssd1306::mode::TerminalModeError;

pub trait TerminalDisplayExt {
    fn write_text(&mut self, s: &str) -> Result<(), TerminalModeError>;
}

impl TerminalDisplayExt for TerminalDisplay {
    fn write_text(&mut self, s: &str) -> Result<(), TerminalModeError> {
        for c in s.chars() {
            self.print_char(c)?;
        }
        Ok(())
    }
}