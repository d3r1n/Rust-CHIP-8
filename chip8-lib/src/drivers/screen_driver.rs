use sdl2::Sdl;
// Import SDL2
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

// Import constants
use crate::constants::*;

// Colors as SDL2 Color structs
const SDL_BACK_COLOR: Color = Color::RGB(0x0E, 0x0F, 0x12);
const SDL_FORE_COLOR: Color = Color::RGB(0x35, 0xD6, 0x2F);

// Define the Screen struct
pub struct Screen {
    pub canvas: WindowCanvas,
}

// Implement the Screen struct
impl Screen {
    // Create a new screen
    pub fn new() -> (Self, Sdl) {
        // Create a new SDL2 context
        let sdl_context = sdl2::init().unwrap();
        // Create a new video context
        let video_subsystem = sdl_context.video().unwrap();
        // Create a new window
        let window = video_subsystem
            .window(
                "CHIP-8 Emulator",
                SCREEN_WIDTH * SCREEN_SCALE,
                SCREEN_HEIGHT * SCREEN_SCALE,
            )
            .position_centered()
            .build()
            .unwrap();
        // Create a new canvas
        let mut canvas = window.into_canvas().build().unwrap();
        // Set the canvas draw color to back color
        canvas.set_draw_color(SDL_BACK_COLOR);
        // Clear the canvas
        canvas.clear();
        // Scale the canvas
        canvas
            .set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32)
            .unwrap();
        // Return the new screen
        (Screen { canvas }, sdl_context)
    }

    pub fn draw_screen(&mut self, screen: &[bool; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]) {
        // Set the canvas draw color to back color
        self.canvas.set_draw_color(SDL_BACK_COLOR);
        // Clear the canvas
        self.canvas.clear();
        // Set the canvas draw color to fore color
        self.canvas.set_draw_color(SDL_FORE_COLOR);
        // Draw the screen
        for (i, pixel) in screen.iter().enumerate() {
            if *pixel {
                let x = i % (SCREEN_WIDTH as usize);
                let y = i / (SCREEN_WIDTH as usize);
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
                    .unwrap();
            }
        }
        // Present the canvas
        self.canvas.present();
    }

    // Clear the screen
    pub fn clear(&mut self) {
        // Set the canvas draw color to black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        // Clear the canvas
        self.canvas.clear();
    }

    // Update the screen
    pub fn update(&mut self) {
        // Update the canvas
        self.canvas.present();
    }
}
