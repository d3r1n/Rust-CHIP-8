use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

pub struct AudioDriver;

impl AudioDriver {
    pub fn play_bip() {
        AudioDriver::play("assets/bip.wav", 100);
    }

    pub fn play_open() {
        AudioDriver::play("assets/open.wav", 850);
    }

    pub fn play_close() {
        AudioDriver::play("assets/close.wav", 800);
    }

    pub fn play_err() {
        AudioDriver::play("assets/err.wav", 100);
    }

    fn play(path: &str, millis: u64) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let file = BufReader::new(File::open(path).unwrap());

        let source = Decoder::new(file).unwrap();

        stream_handle.play_raw(source.convert_samples()).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(millis));
    }
}
