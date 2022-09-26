use rodio::{source::Source, Decoder, OutputStream};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct AudioDriver<'a> {
	pub config: HashMap<&'a str, &'a str>
}

impl<'a> AudioDriver<'a> {
    pub fn play_bip(&self) {
        AudioDriver::play(self.config.get("bip").unwrap(), 100);
    }

    pub fn play_open(&self) {
        AudioDriver::play(self.config.get("open").unwrap(), 850);
    }

    pub fn play_close(&self) {
        AudioDriver::play(self.config.get("close").unwrap(), 800);
    }

    pub fn play_err(&self) {
        AudioDriver::play(self.config.get("error").unwrap(), 100);
    }

    fn play(path: &str, millis: u64) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let file = BufReader::new(File::open(path).unwrap());

        let source = Decoder::new(file).unwrap();

        stream_handle.play_raw(source.convert_samples()).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(millis));
    }
}
