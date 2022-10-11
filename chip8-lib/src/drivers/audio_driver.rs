use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

pub struct AudioDriver {
	pub audio_files: [String; 4]
}

impl AudioDriver {
	pub fn new(audio_files: [String; 4]) -> Self {
		AudioDriver {
			audio_files
		}
	}

    pub fn play_bip(&self) {
        AudioDriver::play(self.audio_files[0].as_str(), 100);
    }

    pub fn play_open(&self) {
        AudioDriver::play(self.audio_files[1].as_str(), 850);
    }

    pub fn play_close(&self) {
        AudioDriver::play(self.audio_files[2].as_str(), 800);
    }

    pub fn play_err(&self) {
        AudioDriver::play(self.audio_files[3].as_str(), 100);
    }

    fn play(path: &str, millis: u64) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let file = BufReader::new(File::open(path).unwrap());

        let source = Decoder::new(file).unwrap();

        stream_handle.play_raw(source.convert_samples()).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(millis));
    }
}
