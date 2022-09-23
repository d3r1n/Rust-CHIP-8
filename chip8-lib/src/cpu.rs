// NNN or address - A 12-bit value, the lowest 12 bits of the instruction
// NN or byte - An 8-bit value, the lowest 8 bits of the instruction
// N or nibble - A 4-bit value, the lowest 4 bits of the instruction
// X or X register - A 4-bit value, the lower 4 bits of the high byte of the instruction
// Y or Y register - A 4-bit value, the upper 4 bits of the low byte of the instruction
pub struct OpCode(u16);

#[allow(dead_code)]
impl OpCode {
	pub fn new(op: u16) -> OpCode {
		OpCode(op)
	}

	// Get the X register of the opcode: 0x0X00
	pub fn oxoo(&self) -> usize {
		((self.0 & 0x0F00) >> 8) as usize
	}

	// Get the Y register of the opcode: 0x00Y0
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

impl From<u16> for OpCode {
	fn from(op: u16) -> OpCode {
		OpCode(op)
	}
}

pub type Register = usize;
pub type Address = u16; // original address value is 12 bits, but we have to use 16 bits to store it

// All of the standart instructions in CHIP-8
pub enum Instruction {
	ClearScreen,						// 00E0 - CLS
	Return,								// 00EE - RET

	Jump(Address),						// 1NNN - JP addr
	Call(Address),						// 2NNN - CALL addr
	SkipEqual(Register, u8), 			// 3XNN - SE Vx, byte
	SkipNotEqual(Register, u8), 		// 4XNN - SNE Vx, byte
	SkipEqualXY(Register, Register), 	// 5XY0 - SE Vx, Vy
	Load(Register, u8), 				// 6XNN - LD Vx, byte
	Add(Register, u8), 					// 7XNN - ADD Vx, byte

	LoadXY(Register, Register), 		// 8XY0 - LD Vx, Vy
	Or(Register, Register),				// 8XY1 - OR Vx, Vy
	And(Register, Register), 			// 8XY2 - AND Vx, Vy
	Xor(Register, Register), 			// 8XY3 - XOR Vx, Vy
	AddXY(Register, Register), 			// 8XY4 - ADD Vx, Vy
	SubXY(Register, Register), 			// 8XY5 - SUB Vx, Vy
	ShiftRight(Register), 				// 8XY6 - SHR Vx {, Vy}
	SubNXY(Register, Register), 		// 8XY7 - SUBN Vx, Vy
	ShiftLeft(Register), 				// 8XYE - SHL Vx {, Vy}

	SkipNotEqualXY(Register, Register), // 9XY0 - SNE Vx, Vy
	LoadI(Address), 					// ANNN - LD I, addr
	JumpV0(Address), 					// BNNN - JP V0, addr
	Rand(Register, u8), 				// CXNN - RND Vx, byte
	Draw(Register, Register, u8), 		// DXYN - DRW Vx, Vy, nibble

	SkipKeyPressed(Register), 			// EX9E - SKP Vx
	SkipNotKeyPressed(Register), 		// EXA1 - SKNP Vx
	
	LoadDelay(Register), 				// FX07 - LD Vx, DT
	WaitKeyPress(Register), 			// FX0A - LD Vx, K
	SetDelay(Register), 				// FX15 - LD DT, Vx
	SetSound(Register), 				// FX18 - LD ST, Vx
	AddI(Register), 					// FX1E - ADD I, Vx
	LoadSprite(Register), 				// FX29 - LD F, Vx
	LoadBCD(Register), 					// FX33 - LD B, Vx
	LoadRegisters(Register), 			// FX55 - LD [I], Vx
	LoadMemory(Register), 				// FX65 - LD Vx, [I]
}

impl Instruction {
	// Return an instruction from an opcode
	pub fn from<I: Into<OpCode>>(opcode: I) -> Option<Instruction> {
		let opcode: OpCode = opcode.into();
		match opcode.0 & 0xF000 { // Match the first nibble
			0x0000 => match opcode.0 & 0x000F { // Match the instructions starting with 0x0
				0x0000 => Some(Instruction::ClearScreen),
				0x000E => Some(Instruction::Return),
				_ => None,
			},

			0x1000 => Some(Instruction::Jump(opcode.onnn())),
			0x2000 => Some(Instruction::Call(opcode.onnn())),
			0x3000 => Some(Instruction::SkipEqual(opcode.oxoo(), opcode.oonn())),
			0x4000 => Some(Instruction::SkipNotEqual(opcode.oxoo(), opcode.oonn())),
			0x5000 => Some(Instruction::SkipEqualXY(opcode.oxoo(), opcode.ooyo())),
			0x6000 => Some(Instruction::Load(opcode.oxoo(), opcode.oonn())),
			0x7000 => Some(Instruction::Add(opcode.oxoo(), opcode.oonn())),

			0x8000 => match opcode.0 & 0x000F { // Match the instructions starting with 0x8
				0x0000 => Some(Instruction::LoadXY(opcode.oxoo(), opcode.ooyo())),
				0x0001 => Some(Instruction::Or(opcode.oxoo(), opcode.ooyo())),
				0x0002 => Some(Instruction::And(opcode.oxoo(), opcode.ooyo())),
				0x0003 => Some(Instruction::Xor(opcode.oxoo(), opcode.ooyo())),
				0x0004 => Some(Instruction::AddXY(opcode.oxoo(), opcode.ooyo())),
				0x0005 => Some(Instruction::SubXY(opcode.oxoo(), opcode.ooyo())),
				0x0006 => Some(Instruction::ShiftRight(opcode.oxoo())),
				0x0007 => Some(Instruction::SubNXY(opcode.oxoo(), opcode.ooyo())),
				0x000E => Some(Instruction::ShiftLeft(opcode.oxoo())),
				_ => None,
			},

			0x9000 => Some(Instruction::SkipNotEqualXY(opcode.oxoo(), opcode.ooyo())),
			0xA000 => Some(Instruction::LoadI(opcode.onnn())),
			0xB000 => Some(Instruction::JumpV0(opcode.onnn())),
			0xC000 => Some(Instruction::Rand(opcode.oxoo(), opcode.oonn())),
			0xD000 => Some(Instruction::Draw(opcode.oxoo(), opcode.ooyo(), opcode.ooon())),
			
			0xE000 => match opcode.0 & 0x00FF { // Match the instructions starting with 0xE
				0x009E => Some(Instruction::SkipKeyPressed(opcode.oxoo())),
				0x00A1 => Some(Instruction::SkipNotKeyPressed(opcode.oxoo())),
				_ => None,
			},

			0xF000 => match opcode.0 & 0x00FF { // Match the instructions starting with 0xF
				0x0007 => Some(Instruction::LoadDelay(opcode.oxoo())),
				0x000A => Some(Instruction::WaitKeyPress(opcode.oxoo())),
				0x0015 => Some(Instruction::SetDelay(opcode.oxoo())),
				0x0018 => Some(Instruction::SetSound(opcode.oxoo())),
				0x001E => Some(Instruction::AddI(opcode.oxoo())),
				0x0029 => Some(Instruction::LoadSprite(opcode.oxoo())),
				0x0033 => Some(Instruction::LoadBCD(opcode.oxoo())),
				0x0055 => Some(Instruction::LoadRegisters(opcode.oxoo())),
				0x0065 => Some(Instruction::LoadMemory(opcode.oxoo())),
				_ => None,
			},
			_ => None			
		}
	}
}