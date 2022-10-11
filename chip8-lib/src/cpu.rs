// NNN or address - A 12-bit value, the lowest 12 bits of the instruction
// NN or byte - An 8-bit value, the lowest 8 bits of the instruction
// N or nibble - A 4-bit value, the lowest 4 bits of the instruction
// X or X register - A 4-bit value, the lower 4 bits of the high byte of the instruction
// Y or Y register - A 4-bit value, the upper 4 bits of the low byte of the instruction
use rand::Rng;

use crate::errors::Chip8Error;
use crate::drivers::{
	graphics_driver::Display,
	keyboard_driver::Keyboard,
	rom_driver::ROM,
	audio_driver::AudioDriver,
};

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
    ClearDisplay, // 00E0 - CLS
    Return,      // 00EE - RET

    Jump(Address),                   // 1NNN - JP addr
    Call(Address),                   // 2NNN - CALL addr
    SkipEqual(Register, u8),         // 3XNN - SE Vx, byte
    SkipNotEqual(Register, u8),      // 4XNN - SNE Vx, byte
    SkipEqualXY(Register, Register), // 5XY0 - SE Vx, Vy
    Load(Register, u8),              // 6XNN - LD Vx, byte
    Add(Register, u8),               // 7XNN - ADD Vx, byte

    LoadXY(Register, Register), // 8XY0 - LD Vx, Vy
    Or(Register, Register),     // 8XY1 - OR Vx, Vy
    And(Register, Register),    // 8XY2 - AND Vx, Vy
    Xor(Register, Register),    // 8XY3 - XOR Vx, Vy
    AddXY(Register, Register),  // 8XY4 - ADD Vx, Vy
    SubXY(Register, Register),  // 8XY5 - SUB Vx, Vy
    ShiftRight(Register),       // 8XY6 - SHR Vx {, Vy}
    SubNXY(Register, Register), // 8XY7 - SUBN Vx, Vy
    ShiftLeft(Register),        // 8XYE - SHL Vx {, Vy}

    SkipNotEqualXY(Register, Register), // 9XY0 - SNE Vx, Vy
    LoadI(Address),                     // ANNN - LD I, addr
    JumpV0(Address),                    // BNNN - JP V0, addr
    Rand(Register, u8),                 // CXNN - RND Vx, byte
    Draw(Register, Register, u8),       // DXYN - DRW Vx, Vy, nibble

    SkipKeyPressed(Register),    // EX9E - SKP Vx
    SkipNotKeyPressed(Register), // EXA1 - SKNP Vx

    LoadDelay(Register),     // FX07 - LD Vx, DT
    WaitKeyPress(Register),  // FX0A - LD Vx, K
    SetDelay(Register),      // FX15 - LD DT, Vx
    SetSound(Register),      // FX18 - LD ST, Vx
    AddI(Register),          // FX1E - ADD I, Vx
    LoadSprite(Register),    // FX29 - LD F, Vx
    LoadBCD(Register),       // FX33 - LD B, Vx
    LoadRegisters(Register), // FX55 - LD [I], Vx
    LoadMemory(Register),    // FX65 - LD Vx, [I]
}

impl Instruction {
    // Return an instruction from an opcode
    pub fn from<I: Into<OpCode>>(opcode: I) -> Option<Instruction> {
        let opcode: OpCode = opcode.into();
        match opcode.0 & 0xF000 {
            // Match the first nibble
            0x0000 => match opcode.0 & 0x000F {
                // Match the instructions starting with 0x0
                0x0000 => Some(Instruction::ClearDisplay),
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

            0x8000 => match opcode.0 & 0x000F {
                // Match the instructions starting with 0x8
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
            0xD000 => Some(Instruction::Draw(
                opcode.oxoo(),
                opcode.ooyo(),
                opcode.ooon(),
            )),

            0xE000 => match opcode.0 & 0x00FF {
                // Match the instructions starting with 0xE
                0x009E => Some(Instruction::SkipKeyPressed(opcode.oxoo())),
                0x00A1 => Some(Instruction::SkipNotKeyPressed(opcode.oxoo())),
                _ => None,
            },

            0xF000 => match opcode.0 & 0x00FF {
                // Match the instructions starting with 0xF
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
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub struct Emulator {
	// 0x000 - 0x1FF: Chip 8 interpreter (contains font set in emu)
	// 0x050 - 0x0A0: Used for the built in 4x5 pixel font set (0-F)
	// 0x200 - 0xFFF: Program ROM and work RAM
	pub memory: [u8; 4096], // 4K memory; 0x000 - 0xFFF
	pub v: [u8; 16], 	   	// 16 8-bit registers; 0x0 - 0xF
	pub i: u16, 			// memory address register
	pub pc: u16, 			// program counter	
	pub stack: [u16; 16], 	// stack; 16 levels of 16-bit values
	pub sp: u8, 			// stack pointer; points to the top of the stack
	pub delay_timer: u8, 	// delay timer
	pub sound_timer: u8, 	// sound timer
	pub display: Display, 	// display
	pub keyboard: Keyboard, // keyboard
	pub audio: AudioDriver, // audio
}

impl Emulator {
	// MISC operations
	pub fn new(audio_files: [String; 4]) -> Self {
		Self {
			memory: [0; 4096],
			v: [0; 16],
			i: 0x200,
			pc: 0x200,
			stack: [0; 16],
			sp: 0,
			delay_timer: 0, 
			sound_timer: 0, 
			display: Display::new(64,32, 0x0E0F12, 0x35D62F),
			keyboard: Keyboard::new(),
			audio: AudioDriver::new(audio_files),
		}
	}
	
	pub fn load_rom(&mut self, rom: ROM) {
		// Load the ROM into memory
		for (i, byte) in rom.data.iter().enumerate() {
			self.memory[0x200 + i] = *byte;
		}
	}

	pub fn reset(&mut self) {
		self.reset_memory();
		self.reset_registers();
	}

	pub fn reset_memory(&mut self) {
		self.memory = [0; 4096];
	}

	pub fn reset_registers(&mut self) {
		self.v = [0; 16];
		self.i = 0x200;
		self.pc = 0x200;
		self.stack = [0; 16];
		self.sp = 0;
		self.delay_timer = 0;
		self.sound_timer = 0;
	}

	// Instruction Operations
	pub fn read_instruction(&self) -> Option<Instruction> {
		let opcode = self.read_opcode();
		Instruction::from(opcode)
	}

	pub fn read_opcode(&self) -> OpCode {
		// Read the 2 byte long opcode from memory
		let hb = self.memory[self.pc as usize] as u16; // high byte (left side byte)
		let lb = self.memory[(self.pc + 1) as usize] as u16; // low byte (right side byte)
		let combine = (hb << 8) | lb; // combine the 2 bytes into a 16 bit opcode
		OpCode(combine)
	}

	pub fn check_register(&self, register: usize) -> Option<Chip8Error> {
		if !register < 16 {
			Some(Chip8Error::InvalidRegister(register))
		}
		else { None }
	}

	pub fn run_instruction(&mut self, instr: Instruction) -> Result<(), Chip8Error> {
		match instr {
			Instruction::ClearDisplay => {
				// Clear the display
				self.display.clear();
				// Increment the program counter by 2
				self.pc += 2;
				Ok(())
			},
			Instruction::Return => {
				// Return from a subroutine
				if self.sp < 1 {
					Err(Chip8Error::StackUnderflow)
				}
				else {
					// Decrement the stack pointer
					self.sp -= 1;
					// Set the program counter to the address at the top of the stack
					self.pc = self.stack[self.sp as usize];
					// Increment the program counter by 2 to skip the returned address
					self.pc += 2;
					Ok(())
				}
			},
			Instruction::Jump(addr) => {
				// Jump to location nnn
				self.pc = addr;
				Ok(())
			},
			Instruction::Call(addr) => {
				// Call subroutine at nnn
				if self.sp > 15 {
					Err(Chip8Error::StackOverflow)
				}
				else {
					// Store the current program counter on the stack
					self.stack[self.sp as usize] = self.pc;
					// Increment the stack pointer
					self.sp += 1;
					// Set the program counter to nnn
					self.pc = addr;
					Ok(())
				}
			},
			Instruction::SkipEqual(reg_x, byte) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Skip next instruction if Vx = byte
				if self.v[reg_x]== byte {
					self.pc += 4;
				}
				else {
					self.pc += 2;
				}

				Ok(())
			},
			Instruction::SkipNotEqual(reg_x, byte) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Skip next instruction if Vx != byte
				// Else increment the program counter by 2
				if self.v[reg_x]!= byte {
					self.pc += 4;
				}
				else {
					self.pc += 2;
				}
				
				Ok(())
			},
			Instruction::SkipEqualXY(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Skip next instruction if Vx = Vy
				// Else increment the program counter by 2
				if self.v[reg_x] == self.v[reg_y] {
					self.pc += 4;
				}
				else {
					self.pc += 2;
				}
				Ok(())
			},
			Instruction::Load(reg_x, byte) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Vx = byte
				self.v[reg_x]= byte;
				
				self.pc += 2;
				Ok(())
			},
			Instruction::Add(reg_x, byte) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Vx += byte
				self.v[reg_x]+= byte;

				self.pc += 2;
				Ok(())
			},
			Instruction::LoadXY(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Vx = Vy
				self.v[reg_x] = self.v[reg_y];

				self.pc += 2;
				Ok(())
			},
			Instruction::Or(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Vx = Vx | Vy (Bitwise OR)
				self.v[reg_x] |= self.v[reg_y];

				self.pc += 2;
				Ok(())
			},
			Instruction::And(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Vx = Vx & Vy (Bitwise AND)
				self.v[reg_x] &= self.v[reg_y];	

				self.pc += 2;
				Ok(())
			},
			Instruction::Xor(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Vx = Vx ^ Vy (Bitwise XOR)
				self.v[reg_x] ^= self.v[reg_y];

				self.pc += 2;
				Ok(())
			},
			Instruction::AddXY(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Vx += Vy
				self.v[reg_x] += self.v[reg_y];

				self.pc += 2;
				Ok(())
			},
			Instruction::SubXY(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Vx -= Vy
				self.v[reg_x] -= self.v[reg_y];

				self.pc += 2;
				Ok(())
			},
			Instruction::ShiftRight(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Shift Vx right and save the least significant bit in register VF
				self.v[0xF] = self.v[reg_x] & 0x1;
				self.v[reg_x] >>= 1;

				self.pc += 2;
				Ok(())
			},
			Instruction::SubNXY(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}
				// If Vx > Vy then VF = 1, otherwise VF = 0
				// Vx = Vy - Vx
				self.v[reg_x] = self.v[reg_y] - self.v[reg_x];

				self.pc += 2;
				Ok(())
			},
			Instruction::ShiftLeft(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Shift Vx left and save the most significant bit in register VF
				self.v[0xF] = self.v[reg_x] >> 7;
				self.v[reg_x] <<= 1;

				self.pc += 2;
				Ok(())
			},
			Instruction::SkipNotEqualXY(reg_x, reg_y) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}

				// Skip next instruction if Vx != Vy
				if self.v[reg_x] != self.v[reg_y] {
					self.pc += 4;
				}
				else {
					self.pc += 2;
				}
				Ok(())
			},
			Instruction::LoadI(addr) => {
				// I = addr
				self.i = addr;

				self.pc += 2;
				Ok(())
			},
			Instruction::JumpV0(addr) => {
				// Jump to addr + V0
				self.pc = addr + self.v[0] as u16;
				Ok(())
			},
			Instruction::Rand(reg_x, byte) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				
				// Random generator
				let mut rng = rand::thread_rng();

				// Vx = random_u8 & byte
				self.v[reg_x] = rng.gen::<u8>() & byte;

				self.pc += 2;
				Ok(())
			},
			Instruction::Draw(reg_x, reg_y , nibble) => {
				// Check registers
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}
				else if let Some(err) = self.check_register(reg_y) {
					return Err(err);
				}
				// Check nibble
				if nibble > 0xF {
					return Err(Chip8Error::InvalidNibble(nibble));
				}

				// Draw sprite at (Vx, Vy) with height nibble
				// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
				let sprite_start = self.i as usize;
				let sprite_end = sprite_start + nibble as usize;

				let x = self.v[reg_x] as usize;
				let y = self.v[reg_y] as usize;

				self.v[0xF] = self
					.display
					.draw(x, y, &self.memory[sprite_start..sprite_end]);

				self.pc += 2;
				Ok(())
			},
			Instruction::SkipKeyPressed(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Skip next instruction if key with the value of Vx is pressed				
				if self.keyboard.pressed_key() == Some(self.v[reg_x]) {
					self.pc += 4;
				}
				else {
					self.pc += 2;
				} 

				Ok(())
			},
			Instruction::SkipNotKeyPressed(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Skip next instruction if key with the value of Vx is not pressed
				if self.keyboard.pressed_key() != Some(self.v[reg_x]) {
					self.pc += 4;
				}
				else {
					self.pc += 2;
				}

				Ok(())
			},
			Instruction::LoadDelay(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Vx = delay timer value
				self.v[reg_x] = self.delay_timer;

				self.pc += 2;
				Ok(())
			},
			Instruction::WaitKeyPress(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Wait for a key press, store the value of the key in Vx
				if let Some(key) = self.keyboard.pressed_key() {
					self.v[reg_x] = key;
					self.pc += 2;
				}

				Ok(())
			},
			Instruction::SetDelay(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Delay timer = Vx
				self.delay_timer = self.v[reg_x];

				self.pc += 2;
				Ok(())
			},
			Instruction::SetSound(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Sound timer = Vx
				self.sound_timer = self.v[reg_x];

				self.pc += 2;
				Ok(())
			},
			Instruction::AddI(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// I += Vx
				self.i += self.v[reg_x] as u16;

				self.pc += 2;
				Ok(())
			},
			Instruction::LoadSprite(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// I = location of sprite for digit Vx
				self.i = self.v[reg_x] as u16 * 5;

				self.pc += 2;
				Ok(())
			},
			Instruction::LoadBCD(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Store BCD representation of Vx in memory locations I, I+1, and I+2
				let value = self.v[reg_x];
				self.memory[self.i as usize] = value / 100;
				self.memory[self.i as usize + 1] = (value / 10) % 10;
				self.memory[self.i as usize + 2] = (value % 100) % 10;
				
				self.pc += 2;
				Ok(())
			},
			Instruction::LoadRegisters(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Store registers V0 through Vx in memory starting at location I
				for i in 0..=reg_x {
					self.v[i] = self.memory[self.i as usize + i];
				}

				self.i += (reg_x as u16) + 1;

				self.pc += 2;
				Ok(())
			},
			Instruction::LoadMemory(reg_x) => {
				// Check register
				if let Some(err) = self.check_register(reg_x) {
					return Err(err);
				}

				// Read registers V0 through Vx from memory starting at location I
				for i in 0..=reg_x {
					self.memory[self.i as usize + i] = self.v[i];
				}

				self.i += (reg_x as u16) + 1;

				self.pc += 2;
				Ok(())
			}
		}
	}
}