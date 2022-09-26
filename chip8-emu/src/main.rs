use chip8_lib::drivers::audio_driver::AudioDriver;

fn main() {
	let ad = AudioDriver {
		config: vec![
			("bip", "bip.wav"),
			("open", "open.wav"),
			("close", "close.wav"),
			("error", "error.wav")
		].into_iter().collect()
	};
	ad.play_open();
    ad.play_bip();
    std::thread::sleep(std::time::Duration::from_secs(1));
    ad.play_err();
    std::thread::sleep(std::time::Duration::from_secs(1));
    ad.play_close();
}
