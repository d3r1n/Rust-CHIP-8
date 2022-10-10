use std::fmt;
pub enum Chip8Error {
	StackUnderflow,
	StackOverflow,
	InvalidRegister(usize),
	InvalidNibble(u8),
}

impl std::fmt::Display for Chip8Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Chip8Error::StackUnderflow => write!(f, "Stack Underflow"),
			Chip8Error::StackOverflow => write!(f, "Stack Overflow"),
			Chip8Error::InvalidRegister(register) => write!(f, "Invalid Register: {}", register),
			Chip8Error::InvalidNibble(nibble) => write!(f, "Invalid Nibble: {}", nibble),
		}
	}
}