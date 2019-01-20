use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

pub fn read_file(filepath: &str) -> Result<String, Error> {
    File::open(filepath).and_then(|file| {
        let mut buffered_reader = BufReader::new(file);
        let mut contents = String::new();
        buffered_reader.read_to_string(&mut contents).map(|_| contents.to_owned())
    })
}

pub fn write_file(filepath: &str, content: String) -> Result<(), Error> {
    File::create(filepath).and_then(|mut file| {
        file.write_all(content.as_bytes())
    })
}
