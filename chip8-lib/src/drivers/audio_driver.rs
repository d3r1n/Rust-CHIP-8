use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{cpal, Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

const BEEP_PATH: &str = "./assets/beep.wav";
const ERROR_PATH: &str = "./assets/error.wav";
const OPENING_PATH: &str = "./assets/opening.wav";
const CLOSE_PATH: &str = "./assets/close.wav";

pub enum Sounds {
    Beep,
    Error,
    Opening,
    Closing,
}

pub struct AudioDriver {
    pub sink: Sink,
}

impl AudioDriver {
    pub fn new() -> Self {
        fn list_devices() {
            let host = cpal::default_host();
            let devices = host.output_devices().unwrap();
            for device in devices {
                let dev: rodio::Device = device.into();
                let dev_name: String = dev.name().unwrap();
                println!(" # Device : {}", dev_name);
            }
        }

        fn get_handler(device_name: &str) -> (OutputStream, OutputStreamHandle) {
            let host = cpal::default_host();
            let devices = host.output_devices().unwrap();
            let (mut _stream, mut stream_handle) = OutputStream::try_default().unwrap();
            for device in devices {
                let dev: rodio::Device = device.into();
                let dev_name: String = dev.name().unwrap();
                if dev_name == device_name {
                    println!("Device found: {}", dev_name);
                    (_stream, stream_handle) = OutputStream::try_from_device(&dev).unwrap();
                }
            }
            return (_stream, stream_handle);
        }

        list_devices();
        let (_stream, stream_handle) = get_handler("MacBook Pro Speakers");

        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioDriver { sink }
    }

    fn append_sink(&self, sound: Sounds) {
        let file = match sound {
            Sounds::Beep => File::open(BEEP_PATH).unwrap(),
            Sounds::Error => File::open(ERROR_PATH).unwrap(),
            Sounds::Opening => File::open(OPENING_PATH).unwrap(),
            Sounds::Closing => File::open(CLOSE_PATH).unwrap(),
        };
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
    }

    pub fn play(&self, sound: Sounds) {
        self.append_sink(sound);
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn volume(&self, volume: f32) {
        if volume > 1.0 {
            self.sink.set_volume(1.0);
        } else if volume < 0.0 {
            self.sink.set_volume(0.0);
        } else {
            self.sink.set_volume(volume);
        }
    }
}
