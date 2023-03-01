//! Module for the VSCAD visual scroll offset instruction constructors

use crate::Error;

use super::DcsCommand;

/// Set Scroll Start
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetScrollStart(u16);

impl SetScrollStart {
    /// Creates a new Set Scroll Start command.
    pub fn new(offset: u16) -> Self {
        Self(offset)
    }
}

impl DcsCommand for SetScrollStart {
    fn instruction(&self) -> u8 {
        0x37
    }

    fn fill_params_buf(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        let bytes = self.0.to_be_bytes();
        buffer[0] = bytes[0];
        buffer[1] = bytes[1];

        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vscad_fills_offset_properly() -> Result<(), Error> {
        let vscad = SetScrollStart::new(320);

        let mut buffer = [0u8; 2];
        assert_eq!(vscad.fill_params_buf(&mut buffer)?, 2);
        assert_eq!(buffer, [0x1, 0x40]);

        Ok(())
    }
}
