struct Display {

}

impl Display {
	pub fn new() -> Display {
		Display {}
	}

	pub fn clear(&self) {}

	pub fn draw(&self, x: u8, y: u8, value: bool) {}

	pub fn draw_sprite(&self, x: u8, y: u8, sprite: Vec<u8>) {}
}