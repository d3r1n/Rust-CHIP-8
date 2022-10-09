use std::fmt;
pub enum Chip8Error {
	StackUnderflow,
	StackOverflow,
}

impl std::fmt::Display for Chip8Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Chip8Error::StackUnderflow => write!(f, "Stack Underflow"),
			Chip8Error::StackOverflow => write!(f, "Stack Overflow"),
		}
	}
}