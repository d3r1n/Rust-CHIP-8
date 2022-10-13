use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub struct ROM {
    pub data: Vec<u8>,
    pub name: String,
}

impl ROM {
    pub fn new(data: Vec<u8>, name: String) -> ROM {
        ROM { data, name }
    }

    pub fn from_file(file_path: &str) -> io::Result<ROM> {
        let path = Path::new(file_path);
        let mut file = File::open(path).unwrap();
        let mut data: Vec<u8> = Vec::new();

        file.read_to_end(&mut data).unwrap();
        let name: String = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .split('.')
            .next()
            .unwrap()
            .to_string();

        Ok(ROM::new(data, name))
    }
}
