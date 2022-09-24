use chip8_lib::drivers::audio_driver::AudioDriver;

fn main() {
    AudioDriver::play_open();
    AudioDriver::play_bip();
    std::thread::sleep(std::time::Duration::from_secs(1));
    AudioDriver::play_err();
    std::thread::sleep(std::time::Duration::from_secs(1));
    AudioDriver::play_close();
}
