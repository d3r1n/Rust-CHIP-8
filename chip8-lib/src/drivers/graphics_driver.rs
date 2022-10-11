use minifb::{Key, Scale, Window, WindowOptions};

use crate::errors::Chip8Error;

pub struct Display {
	pub pixels: Vec<u8>,
	pub window: Window,
	pub width: usize,
	pub height: usize,
	pub back_color: u32,
	pub fore_color: u32,
}

impl Display {
	pub fn new(width: usize, height: usize, back_color: u32, fore_color: u32) -> Self {
		let mut window = Window::new(
			"Rust CHIP-8",
			width,
			height,
			WindowOptions {
				scale: Scale::X2,
				..WindowOptions::default()
			},
		).unwrap_or_else(|e| {
			panic!("Something went wrong when creating a window: {}", e);
		});

		window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

		Display {
			pixels: vec![0; width * height],
			window,
			width,
			height,
			back_color,
			fore_color,
		}
	}

	pub fn is_open(&self) -> bool {
		self.window.is_open() && !self.window.is_key_down(Key::Escape)
	}

	pub fn clear(&mut self) {
		self.pixels = vec![0; self.width * self.height];
	}

	pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
		let mut collision = 0;
		let mut yj: usize;
		let mut xi: usize;

		for (j, byte) in sprite.iter().enumerate() {
			yj = (y + j) % self.height;

			for i in 0..8 {
				xi = (x + i) % self.width;

				if (byte & (0x80 >> i)) != 0 {
					if self.pixels[yj * self.width + xi] == 1 {
						collision = 1;
					}

					self.pixels[yj * self.width + xi] ^= 1;
				}
			}
		}
		if let Err(e) = self.update_screen() {
			panic!("Something went wrong when updating the screen: {}", e);
		}
		collision
	}

	pub fn update_screen(&mut self) -> Result<(), Chip8Error> {
		let mut buffer: Vec<u32> = vec![0; self.width * self.height];

		for (i, pixel) in self.pixels.iter().enumerate() {
			buffer[i] = if *pixel == 0 { self.back_color } else { self.fore_color };
		}

		self.window
			.update_with_buffer(&buffer, self.width, self.height)
			.map_err(|e| Chip8Error::DisplayError(e.to_string()))
	}
}

impl AsMut<Window> for Display {
	fn as_mut(&mut self) -> &mut Window {
		&mut self.window
	}
}