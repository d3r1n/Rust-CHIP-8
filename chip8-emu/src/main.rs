// Import SDL2
use chip8_lib::{
    cpu::Emulator,
	constants::CLOCK_SPEED,
    drivers::{rom_driver::ROM, screen_driver::Screen},
};
use sdl2::event::Event;
use sdl2::{self, keyboard::Keycode};

fn main() {
    // Initialize SDL2
    let (mut screen, sdl_context) = Screen::new();
    let rom = ROM::from_file("roms/Tetris.ch8").unwrap();

    // Initialize the emulator
    let mut emulator = Emulator::new();

    // Load the ROM into the emulator
    emulator.load_rom(rom);

    // Create an event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Main loop
    'running: loop {
        // Emulator cycle
        emulator.tick().unwrap();
        emulator.timer_tick();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // Handle key presses
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = map_sdl_keys(key) {
                        emulator.key_down(key);
                    }
                }
                // Handle key releases
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = map_sdl_keys(key) {
                        emulator.key_up(key);
                    }
                }
                _ => {}
            }
        }

        // Draw the screen
        if emulator.draw_flag {
            screen.draw_screen(&emulator.screen);
            emulator.draw_flag = false;
        }

        // Handle audio
        if emulator.st > 0 {
            // Play Sound
        }

        // Sleep according to the clock speed
		std::thread::sleep(std::time::Duration::from_millis(CLOCK_SPEED));
	}
}

fn map_sdl_keys(key: Keycode) -> Option<u8> {
    match key {
		Keycode::Num1 => Some(0x1),
		Keycode::Num2 => Some(0x2),
		Keycode::Num3 => Some(0x3),
		Keycode::Num4 => Some(0xC),
		Keycode::Q => Some(0x4),
		Keycode::W => Some(0x5),
		Keycode::E => Some(0x6),
		Keycode::R => Some(0xD),
		Keycode::A => Some(0x7),
		Keycode::S => Some(0x8),
		Keycode::D => Some(0x9),
		Keycode::F => Some(0xE),
		Keycode::Z => Some(0xA),
		Keycode::X => Some(0x0),
		Keycode::C => Some(0xB),
		Keycode::V => Some(0xF),
		_ => None,
	}
}
