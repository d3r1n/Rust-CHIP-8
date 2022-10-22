use chip8_lib::{
	drivers::rom_driver::ROM,
	cpu::Emulator,
	// errors::Chip8Error,
};

fn main() {
    let rom = ROM::from_file("./roms/Tetris.ch8").unwrap();
	let mut emulator = Emulator::new();
	emulator.load_rom(rom);

	while emulator.display.is_open() {
		let keys =  emulator.display.get_window().get_keys();
		keys.iter().for_each(|key| {
			emulator.keyboard.press_key(*key);
		});

		if keys.is_empty() {
			emulator.keyboard.release_key();
		}

		emulator.tick().unwrap();
		emulator.timer_tick();

		std::thread::sleep(std::time::Duration::from_micros(2000));
	}
}
