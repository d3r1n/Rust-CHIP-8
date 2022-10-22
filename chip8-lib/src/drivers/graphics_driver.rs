use minifb::{Key, Scale, Window, WindowOptions};

use crate::errors::Chip8Error;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
pub struct Display {
    pub pixels: [[u32; WIDTH]; HEIGHT],
    pub window: Window,
    pub back_color: u32,
    pub fore_color: u32,
}

impl Display {
    pub fn new(width: usize, height: usize, back_color: u32, fore_color: u32) -> Self {
        let window = Window::new(
            "Rust CHIP-8",
            width,
            height,
            WindowOptions {
                scale: Scale::X8,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("Something went wrong when creating a window: {}", e);
        });

        Display {
            pixels: [[0; WIDTH]; HEIGHT],
            window,
            back_color,
            fore_color,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn clear(&mut self) {
        self.pixels = [[0; WIDTH]; HEIGHT];
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0;
        let mut yj: usize;
        let mut xi: usize;

        for (j, sprite) in sprite.iter().enumerate() {
			for i in 0..8 {
                xi = (x + i) % WIDTH;
                yj = (y + j) % HEIGHT;

                if sprite & (0x80 >> i) != 0 {
                    if self.pixels[yj][xi] == 1 {
                        collision = 1
                    }
                    self.pixels[yj][xi] ^= 1;
                }
            }
        }
        self.update_screen().unwrap();
        collision
    }

    pub fn update_screen(&mut self) -> Result<(), Chip8Error> {
        let mut buffer = [0; WIDTH * HEIGHT];
		let mut i = 0;

		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				buffer[i] = if self.pixels[y][x] == 1 {
					self.fore_color
				} else {
					self.back_color
				};
				i += 1;
			}
		}

        self.window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .map_err(|e| Chip8Error::DisplayError(e.to_string()))
    }

    pub fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }
}
