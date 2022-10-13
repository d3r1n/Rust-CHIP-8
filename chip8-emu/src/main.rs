use chip8_lib::cpu::Emulator;
use chip8_lib::drivers::rom_driver::ROM;
fn main() {
    let mut emulator = Emulator::new();
    emulator.load_rom(ROM::from_file("./roms/INVADERS.ch8").unwrap());
    emulator.emulate();
}
