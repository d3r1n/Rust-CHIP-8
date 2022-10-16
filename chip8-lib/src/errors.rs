use std::fmt;

#[derive(Debug)]
pub enum Chip8Error {
    StackUnderflow,
    StackOverflow,
    InvalidRegister(usize),
    InvalidNibble(u8),
    DisplayError(String),
	InvalidInstruction(u16),
}

impl std::fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chip8Error::StackUnderflow => write!(f, "Stack Underflow"),
            Chip8Error::StackOverflow => write!(f, "Stack Overflow"),
            Chip8Error::InvalidRegister(register) => write!(f, "Invalid Register: {}", register),
            Chip8Error::InvalidNibble(nibble) => write!(f, "Invalid Nibble: {}", nibble),
            Chip8Error::DisplayError(ref e) => write!(f, "Display Error: {}", e),
			Chip8Error::InvalidInstruction(pc) => write!(f, "Invalid Instruction @ PC: {}", pc),
		}
    }
}
