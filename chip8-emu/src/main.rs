use minifb::{Key, Window, WindowOptions};
use chip8_lib::{
	drivers::rom_driver::ROM,
	cpu::*,
	// errors::Chip8Error,
};

fn main() {		
    let rom = ROM::from_file("./roms/INVADERS.ch8").unwrap();
	let mut emulator = Emulator::new();
	emulator.load_rom(rom);

	// Initialize the window
	let mut window = Window::new(
		"Chip8 Emulator",
		DISPLAY_WIDTH,
		DISPLAY_HEIGHT,
		WindowOptions {
			scale: minifb::Scale::X16,
			title: true,
			resize: false,
			..WindowOptions::default()
		}).unwrap_or_else(|e| {
			panic!("{}", e);
		});

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let keys =  window.get_keys();
		keys.iter().for_each(|key| {
			emulator.keyboard.press_key(*key);
		});

		if keys.is_empty() {
			emulator.keyboard.release_key();
		}

		emulator.tick().unwrap();
		emulator.timer_tick();

		if emulator.draw_flag {
			let b_vec: Vec<u32> = emulator.screen.iter().map(|&pixel| {
				if pixel {
					FORE_COLOR
				} else {
					BACK_COLOR
				}
			}).collect();
			let buffer: &[u32] = &b_vec;
			window.update_with_buffer(buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
			emulator.draw_flag = false;
		}

		std::thread::sleep(std::time::Duration::from_millis(5));
	}
}
