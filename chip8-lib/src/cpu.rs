// NNN or addr 		- A 12-bit value, the lowest 12 bits of the instruction
// NN or byte 		- An 8-bit value, the lowest 8 bits of the instruction
// N or nibble 		- A 4-bit value, the lowest 4 bits of the instruction
// X or X register 	- A 4-bit value, the lower 4 bits of the high byte of the instruction
// Y or Y register 	- A 4-bit value, the upper 4 bits of the low byte of the instruction
use rand::Rng;

use crate::drivers::{
    keyboard_driver::Keyboard,
    rom_driver::ROM,
};
use crate::errors::Chip8Error;

// CHIP-8's default fontset
pub const FONT_SET_SIZE: usize = 80;
pub const FONT_SET: [u8; FONT_SET_SIZE] = [
	0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

// CHIP-8's constants
pub const STACK_SIZE: usize = 16;
pub const ROM_START: u16 = 0x200;
pub const MEMORY_SIZE: usize = 4096;
pub const NUM_REGISTERS: usize = 16;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const BACK_COLOR: u32 = 0x0E0F12;
pub const FORE_COLOR: u32 = 0x35D62F;

pub struct OpCode(u16);

#[allow(dead_code)]
impl OpCode {
    pub fn new(op: u16) -> OpCode {
        OpCode(op)
    }

    // Get the X register of the opcode: 0x0X00
    pub fn x(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    // Get the Y register of the opcode: 0x00Y0
    fn y(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    // Get the third nibble of the opcode: 0x000N
    fn n(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    // Get the last two nibbles of the opcode: 0x00NN
    fn nn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    // Get the last three nibbles of the opcode: 0x0NNN
    fn nnn(&self) -> u16 {
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
    Return,       // 00EE - RET

    Jump(Address),                   // 1NNN - JP addr
    Call(Address),                   // 2NNN - CALL addr
    SkipEqual(Register, u8),         // 3XNN - SE Vx, byte
    SkipNotEqual(Register, u8),      // 4XNN - SNE Vx, byte
    SkipEqualXY(Register, Register), // 5XY0 - SE Vx, Vy
    Load(Register, u8),              // 6XNN - LD Vx, byte
    Add(Register, u8),               // 7XNN - ADD Vx, byte

    Move(Register, Register),  // 8XY0 - LD Vx, Vy
    Or(Register, Register),    // 8XY1 - OR Vx, Vy
    And(Register, Register),   // 8XY2 - AND Vx, Vy
    Xor(Register, Register),   // 8XY3 - XOR Vx, Vy
    AddXY(Register, Register), // 8XY4 - ADD Vx, Vy
    SubXY(Register, Register), // 8XY5 - SUB Vx, Vy
    ShiftRight(Register),      // 8XY6 - SHR Vx {, Vy}
    SubYX(Register, Register), // 8XY7 - SUBN Vx, Vy
    ShiftLeft(Register),       // 8XYE - SHL Vx {, Vy}

    SkipNotEqualXY(Register, Register), // 9XY0 - SNE Vx, Vy
    LoadI(Address),                     // ANNN - LD I, addr
    JumpV0(Address),                    // BNNN - JP V0, addr
    Random(Register, u8),               // CXNN - RND Vx, byte
    Draw(Register, Register, u8),       // DXYN - DRW Vx, Vy, nibble

    SkipKeyPressed(Register),    // EX9E - SKP Vx
    SkipKeyNotPressed(Register), // EXA1 - SKNP Vx

    LoadDelay(Register),      // FX07 - LD Vx, DT
    WaitKeyPress(Register),   // FX0A - LD Vx, K
    SetDelay(Register),       // FX15 - LD DT, Vx
    SetSound(Register),       // FX18 - LD ST, Vx
    AddI(Register),           // FX1E - ADD I, Vx
    LoadFont(Register),     // FX29 - LD F, Vx
    StoreBCD(Register),       // FX33 - LD B, Vx
    StoreRegisters(Register), // FX55 - LD [I], Vx
    LoadMemory(Register),     // FX65 - LD Vx, [I]
}

impl Instruction {
    // Return an instruction from an opcode
    pub fn from<I: Into<OpCode>>(opcode: I) -> Option<Instruction> {
        let opcode: OpCode = opcode.into();
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.n() {
                0x0000 => Some(Instruction::ClearDisplay),
                0x000E => Some(Instruction::Return),
                _ => None,
            },

            0x1000 => Some(Instruction::Jump(opcode.nnn())),
            0x2000 => Some(Instruction::Call(opcode.nnn())),
            0x3000 => Some(Instruction::SkipEqual(opcode.x(), opcode.nn())),
            0x4000 => Some(Instruction::SkipNotEqual(opcode.x(), opcode.nn())),
            0x5000 => Some(Instruction::SkipEqualXY(opcode.x(), opcode.y())),
            0x6000 => Some(Instruction::Load(opcode.x(), opcode.nn())),
            0x7000 => Some(Instruction::Add(opcode.x(), opcode.nn())),

            0x8000 => match opcode.n() {
                0x0000 => Some(Instruction::Move(opcode.x(), opcode.y())),
                0x0001 => Some(Instruction::Or(opcode.x(), opcode.y())),
                0x0002 => Some(Instruction::And(opcode.x(), opcode.y())),
                0x0003 => Some(Instruction::Xor(opcode.x(), opcode.y())),
                0x0004 => Some(Instruction::AddXY(opcode.x(), opcode.y())),
                0x0005 => Some(Instruction::SubXY(opcode.x(), opcode.y())),
                0x0006 => Some(Instruction::ShiftRight(opcode.x())),
                0x0007 => Some(Instruction::SubYX(opcode.x(), opcode.y())),
                0x000E => Some(Instruction::ShiftLeft(opcode.x())),
                _ => None,
            },

            0x9000 => Some(Instruction::SkipNotEqualXY(opcode.x(), opcode.y())),
            0xA000 => Some(Instruction::LoadI(opcode.nnn())),
            0xB000 => Some(Instruction::JumpV0(opcode.nnn())),
            0xC000 => Some(Instruction::Random(opcode.x(), opcode.nn())),
            0xD000 => Some(Instruction::Draw(opcode.x(), opcode.y(), opcode.n())),

            0xE000 => match opcode.nn() {
                0x009E => Some(Instruction::SkipKeyPressed(opcode.x())),
                0x00A1 => Some(Instruction::SkipKeyNotPressed(opcode.x())),
                _ => None,
            },

            0xF000 => match opcode.nn() {
                0x0007 => Some(Instruction::LoadDelay(opcode.x())),
                0x000A => Some(Instruction::WaitKeyPress(opcode.x())),
                0x0015 => Some(Instruction::SetDelay(opcode.x())),
                0x0018 => Some(Instruction::SetSound(opcode.x())),
                0x001E => Some(Instruction::AddI(opcode.x())),
                0x0029 => Some(Instruction::LoadFont(opcode.x())),
                0x0033 => Some(Instruction::StoreBCD(opcode.x())),
                0x0055 => Some(Instruction::StoreRegisters(opcode.x())),
                0x0065 => Some(Instruction::LoadMemory(opcode.x())),
                _ => None,
            },
            _ => None,
        }
    }

	pub fn has_register(&self) -> bool {
		match *self {
			Instruction::SkipEqual(_, _) => true,
			Instruction::SkipNotEqual(_, _) => true,
			Instruction::Load(_, _) => true,
			Instruction::Add(_, _) => true,
			Instruction::Move(_, _) => true,
			Instruction::Or(_, _) => true,
			Instruction::And(_, _) => true,
			Instruction::Xor(_, _) => true,
			Instruction::AddXY(_, _) => true,
			Instruction::SubXY(_, _) => true,
			Instruction::ShiftRight(_) => true,
			Instruction::SubYX(_, _) => true,
			Instruction::ShiftLeft(_) => true,
			Instruction::SkipNotEqualXY(_, _) => true,
			Instruction::Random(_, _) => true,
			Instruction::Draw(_, _, _) => true,
			Instruction::SkipKeyPressed(_) => true,
			Instruction::SkipKeyNotPressed(_) => true,
			Instruction::LoadDelay(_) => true,
			Instruction::WaitKeyPress(_) => true,
			Instruction::SetDelay(_) => true,
			Instruction::SetSound(_) => true,
			Instruction::AddI(_) => true,
			Instruction::LoadFont(_) => true,
			Instruction::StoreBCD(_) => true,
			Instruction::StoreRegisters(_) => true,
			Instruction::LoadMemory(_) => true,
			_ => false,
		}
	}
}

#[allow(dead_code)]
pub struct Emulator {
	/* Memory Layout:
		|- 0x000 - 0x1FF: Chip 8 interpreter (contains font set in emulator)
		|- 0x050 - 0x0A0: Used for the built in 4x5 pixel font set (0-F)
		|- 0x200 - 0xFFF: Program ROM and work RAM 
	*/

    pub memory: [u8; MEMORY_SIZE],	// 4K memory; 0x000 - 0xFFF
    pub v: [u8; NUM_REGISTERS],		// 16 8-bit registers; 0x0 - 0xF
    pub i: u16,             		// Memory address register
    pub pc: u16,            		// Program counter
    pub stack: [u16; STACK_SIZE],   // Stack; 16 levels of 16-bit values
    pub sp: u8,             		// Stack pointer; points to the top of the stack
    pub dt: u8,    					// Delay timer
    pub st: u8,    					// Sound timer
    pub keyboard: Keyboard, 		// Keyboard
	pub draw_flag: bool,			// Draw flag
	pub screen: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT], // Screen
}

impl Emulator {
    // MISC operations
    pub fn new() -> Self {
        let mut emulator = Self {
            memory: [0; MEMORY_SIZE],
            v: [0; 16],
            i: 0,
            pc: ROM_START,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            keyboard: Keyboard::new(),
			draw_flag: false,
			screen: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        };

        // Load the font set into memory
        emulator.memory[..FONT_SET_SIZE].copy_from_slice(&FONT_SET);

        emulator
    }

    pub fn load_rom(&mut self, rom: ROM) {
        // Load the ROM into memory
        for (i, byte) in rom.data.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

	fn push(&mut self, val: u16) {
		self.stack[self.sp as usize] = val;
		self.sp += 1;
	}

	fn pop(&mut self) -> u16 {
		self.sp -= 1;
		self.stack[self.sp as usize]
	}

	fn clear_screen(&mut self) {
		self.screen = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
	}

	// Fetch the next instruction
	pub fn fetch(&mut self) -> Option<Instruction> {
		// Read the 2 byte long opcode from memory
        let hb = self.memory[self.pc as usize] as u16; // High byte (left side byte)
        let lb = self.memory[(self.pc + 1) as usize] as u16; // Low byte (right side byte)
        let combine = (hb << 8) | lb; // Combine the 2 bytes into a 16 bit opcode
        let op = OpCode(combine); // Create an opcode from the 16 bit value
		self.pc += 2; // Next instruction
		
		// Turn the opcode into an instruction
		let instruction = Instruction::from(op);
		instruction
	}

	// One cycle of CHIP-8
	pub fn tick(&mut self) -> Result<(), Chip8Error> {
		// Fetch the next instruction
		let instruction = self.fetch();

		match instruction {
			Some(instruction) => {
				// Execute the instruction
				let result = self.execute(instruction);

				if let Err(e) = result {
					return Err(e);
				}
			},
			None => {
				return Err(Chip8Error::InvalidInstruction(self.pc));
			}
		}
		Ok(())
	}

	pub fn execute(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
		match instruction {
			// Clear the display
			Instruction::ClearDisplay => {
				self.clear_screen();
				Ok(())
			},
			// Return from a subroutine
			Instruction::Return => {
				// Check if the stack pointer is 0
				if self.sp == 0 {
					return Err(Chip8Error::StackUnderflow);
				}

				let return_addr = self.pop();
				self.pc = return_addr;
				Ok(())
			},
			// Jump to address
			Instruction::Jump(addr) => {
				self.pc = addr;
				Ok(())
			},
			// Call subroutine at address
			Instruction::Call(addr) => {
				// Check if the stack pointer is at the max
				if self.sp == STACK_SIZE as u8 {
					return Err(Chip8Error::StackOverflow);
				}

				self.push(self.pc);
				self.pc = addr;
				Ok(())
			},
			// Skip next instruction if Vx == byte
			Instruction::SkipEqual(x, byte) => {
				if self.v[x] == byte {
					self.pc += 2;
				}
				Ok(())
			},
			// Skip next instruction if Vx != byte
			Instruction::SkipNotEqual(x, byte) => {
				if self.v[x] != byte {
					self.pc += 2;
				}
				Ok(())
			},
			// Skip next instruction if Vx == Vy
			Instruction::SkipEqualXY(x, y) => {
				if self.v[x] == self.v[y] {
					self.pc += 2;
				}
				Ok(())
			},
			// Set Vx = byte
			Instruction::Load(x, byte) => {
				self.v[x] = byte;
				Ok(())
			},
			// Set Vx = Vx + byte
			Instruction::Add(x, byte) => {
				self.v[x] = self.v[x].wrapping_add(byte);
				Ok(())
			},
			// Set Vx = Vy
			Instruction::Move(x, y) => {
				self.v[x] = self.v[y];
				Ok(())
			},
			// Set Vx = Vx OR Vy
			Instruction::Or(x, y) => {
				self.v[x] |= self.v[y];
				Ok(())
			},
			// Set Vx = Vx AND Vy
			Instruction::And(x, y) => {
				self.v[x] &= self.v[y];
				Ok(())
			},
			// Set Vx = Vx XOR Vy
			Instruction::Xor(x, y) => {
				self.v[x] ^= self.v[y];
				Ok(())
			},
			// Set Vx = Vx + Vy, set VF = carry
			Instruction::AddXY(x, y) => {
				let (val, carry) = self.v[x].overflowing_add(self.v[y]);
				self.v[x] = val;
				self.v[0xF] = carry as u8;
				Ok(())
			},
			// Set Vx = Vx - Vy, set VF = NOT borrow
			Instruction::SubXY(x, y) => {
				let (val, borrow) = self.v[x].overflowing_sub(self.v[y]);
				self.v[x] = val;
				self.v[0xF] = !borrow as u8;
				Ok(())
			},
			// Set Vx = Vx SHR 1
			Instruction::ShiftRight(x) => {
				self.v[0xF] = self.v[x] & 0x1;
				self.v[x] >>= 1;
				Ok(())
			},
			// Set Vx = Vy - Vx, set VF = NOT borrow
			Instruction::SubYX(x, y) => {
				let (val, borrow) = self.v[y].overflowing_sub(self.v[x]);
				self.v[x] = val;
				self.v[0xF] = !borrow as u8;
				Ok(())
			},
			// Set Vx = Vx SHL 1
			Instruction::ShiftLeft(x) => {
				self.v[0xF] = (self.v[x] >> 7) & 0x1;
				self.v[x] <<= 1;
				Ok(())
			},
			// Skip next instruction if Vx != Vy
			Instruction::SkipNotEqualXY(x, y) => {
				if self.v[x] != self.v[y] {
					self.pc += 2;
				}
				Ok(())
			},
			// Set I = addr
			Instruction::LoadI(addr) => {
				self.i = addr;
				Ok(())
			},
			// Jump to location addr + V0
			Instruction::JumpV0(addr) => {
				self.pc = addr + (self.v[0] as u16);
				Ok(())
			},
			// Set Vx = random byte AND byte
			Instruction::Random(x, byte) => {
				let mut rng = rand::thread_rng();
				self.v[x] = rng.gen::<u8>() & byte;
				Ok(())
			},
			// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
			Instruction::Draw(x, y, nibble) => {
				let x_coord = self.v[x] as usize;
				let y_coord = self.v[y] as usize;
				let mut collision = false;

				for row in 0..nibble {
					let addr = (self.i + row as u16) as usize;
					let pixels = self.memory[addr];

					for col in 0..8 {
						if (pixels & (0x80 >> col)) != 0 {
							// Sprites should wrap around screen, so apply modulo
							let x = (x_coord + col) % DISPLAY_WIDTH;
							let y = (y_coord + row as usize) % DISPLAY_HEIGHT;
							// Get our pixel's index for our 1D screen array
							let idx = x + DISPLAY_WIDTH * y;
							// Check if we're about to flip the pixel and set
							collision |= self.screen[idx];
							self.screen[idx] ^= true;
						}
					}
				}

				self.v[0xF] = collision as u8;
				self.draw_flag = true;

				Ok(())
			},
			// Skip next instruction if key with the value of Vx is pressed
			Instruction::SkipKeyPressed(x) => {
				let key = self.v[x];
				if self.keyboard.pressed_key() == Some(key) {
					self.pc += 2;
				}
				Ok(())
			},
			// Skip next instruction if key with the value of Vx is not pressed
			Instruction::SkipKeyNotPressed(x) => {
				let key = self.v[x];
				if self.keyboard.pressed_key() != Some(key) {
					self.pc += 2;
				}	
				Ok(())
			},
			// Set Vx = delay timer value
			Instruction::LoadDelay(x) => {
				self.v[x] = self.dt;
				Ok(())
			},
			// Wait for a key press, store the value of the key in Vx
			Instruction::WaitKeyPress(x) => {
				if let Some(key) = self.keyboard.pressed_key() {
					self.v[x] = key;
				} else {
					self.pc -= 2;
				}
				Ok(())
			},
			// Set delay timer = Vx
			Instruction::SetDelay(x) => {
				self.dt = self.v[x];
				Ok(())
			},
			// Set sound timer = Vx
			Instruction::SetSound(x) => {
				self.st = self.v[x];
				Ok(())
			},
			// Set I = I + Vx
			Instruction::AddI(x) => {
				self.i = self.i.wrapping_add(self.v[x] as u16);
				Ok(())
			},
			// Set I = font sprite for digit Vx
			Instruction::LoadFont(x) => {
				self.i = (self.v[x] as u16) * 5;
				Ok(())
			},
			// Store BCD representation of Vx in memory locations I, I+1, and I+2
			Instruction::StoreBCD(x) => {
				let val = self.v[x] as f32;
				// Get the hundreds digit by dividing by 100 and taking the floor
				let hundreds 	= (val / 100.0).floor() as u8;
				// Get the tens digit by dividing by 10, tossing the ones digit and taking the floor
				let tens 		= ((val / 10.0) % 10.0).floor() as u8;
				// Get the ones digit by taking the remainder of the division by 10
				let ones 		= (val % 10.0).floor() as u8;

				self.memory[self.i as usize] 		= hundreds;
				self.memory[(self.i + 1) as usize] 	= tens;
				self.memory[(self.i + 2) as usize] 	= ones;
				Ok(())
			},
			// Store registers V0 through Vx in memory starting at location I
			Instruction::StoreRegisters(x) => {
				for idx in 0..=x {
					self.memory[(self.i as usize) + idx] = self.v[idx];
				}
				Ok(())
			},
			// Read registers V0 through Vx from memory starting at location I
			Instruction::LoadMemory(x) => {
				for idx in 0..=x {
					self.v[idx] = self.memory[(self.i as usize) + idx];
				}
				Ok(())
			},
		}
	}

	pub fn timer_tick(&mut self) {
		// Decrement delay timer if it's greater than zero every tick
		if self.dt > 0 {
			self.dt -= 1;
		}
	
		// Decrement sound timer if it's greater than zero every tick
		// If it's zero, play the "beep" sound
		if self.st > 0 {
			if self.st == 1 {
				println!("BEEP!");
			}
			self.st -= 1;
		}
	}

	pub fn reset_memory(&mut self) {
		self.memory = [0; MEMORY_SIZE];
	}

	pub fn reset_registers(&mut self) {
		self.v = [0; NUM_REGISTERS];
		self.i = 0;
		self.pc = ROM_START;
	}

	pub fn reset_timers(&mut self) {
		self.dt = 0;
		self.st = 0;
	}

	pub fn reset_stack(&mut self) {
		self.stack = [0; STACK_SIZE];
		self.sp = 0;
	}
}
