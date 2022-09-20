pub struct OpCode(u16);

#[allow(dead_code)]
impl OpCode {
	pub fn new(op: u16) -> OpCode {
		OpCode(op)
	}

	// Get the first nibble of the opcode: 0x0X00
	pub fn oxoo(&self) -> usize {
		((self.0 & 0x0F00) >> 8) as usize
	}

	// Get the second nibble of the opcode: 0x00Y0
	fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

	// Get the third nibble of the opcode: 0x000N
    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

	// Get the last two nibbles of the opcode: 0x00NN
    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

	// Get the last three nibbles of the opcode: 0x0NNN
    fn onnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}