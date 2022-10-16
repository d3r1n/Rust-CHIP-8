use chip8_lib::{
	drivers::rom_driver::ROM,
	cpu::Emulator,
	// errors::Chip8Error,
};

// 60 Hz clock
const CLOCK_SPEED: u64 = 60;

fn main() {
    let rom = ROM::from_file("./roms/INVADERS.ch8").unwrap();
	let mut emulator = Emulator::new();
	emulator.load_rom(rom);

	while emulator.display.is_open() {
		emulator.display.update_screen().unwrap();

		let keys =  emulator.display.get_window().get_keys();
		keys.iter().for_each(|key| {
			emulator.keyboard.press_key(*key);
		});

		if keys.is_empty() {
			emulator.keyboard.release_key();
		}

		emulator.tick().unwrap();
		emulator.timer_tick();

		// Clock
		std::thread::sleep(std::time::Duration::from_millis(5));
	}
}
