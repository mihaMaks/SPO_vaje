use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::{self, Read, Write};

// Trait za napravo
pub trait Device {
    fn test(&self) -> bool; // Vzorec za testiranje naprave (vrne true)
    fn read(&mut self) -> u8; // Branje iz naprave (vrne byte)
    fn write(&mut self, value: u8); // Pisanje v napravo
}

// Standardna naprava, ki nič ne počne
pub struct BasicDevice;

impl Device for BasicDevice {
    fn test(&self) -> bool {
        true // Vedno vrne true za testiranje
    }

    fn read(&mut self) -> u8 {
        0 // Vrne 0 pri branju
    }

    fn write(&mut self, _value: u8) {
        // Ne počne nič pri pisanju
    }
}

// Vhodna naprava, ki bere iz vhodnega toka
pub struct InputDevice<R: Read> {
    input_stream: R,
}

impl<R: Read> InputDevice<R> {
    pub fn new(input_stream: R) -> Self {
        Self { input_stream }
    }
}

impl<R: Read> Device for InputDevice<R> {
    fn test(&self) -> bool {
        true
    }

    fn read(&mut self) -> u8 {
        let mut buffer = [0u8; 1];
        match self.input_stream.read(&mut buffer) {
            Ok(_) => buffer[0],
            Err(_) => 0,
        }
    }

    fn write(&mut self, _value: u8) {
        // Ni potrebna implementacija za InputDevice
    }
}

// Izhodna naprava, ki piše v izhodni tok
pub struct OutputDevice<W: Write> {
    output_stream: W,
}

impl<W: Write> OutputDevice<W> {
    pub fn new(output_stream: W) -> Self {
        Self { output_stream }
    }
}

impl<W: Write> Device for OutputDevice<W> {
    fn test(&self) -> bool {
        true
    }

    fn read(&mut self) -> u8 {
        0 // Ne podpira branja
    }

    fn write(&mut self, value: u8) {
        self.output_stream.write_all(&[value]).unwrap();
    }
}

// File naprava, ki bere in piše v datoteko
pub struct FileDevice {
    file: std::fs::File,
}

impl FileDevice {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        Self { file }
    }
}

impl Device for FileDevice {
    fn test(&self) -> bool {
        true
    }

    fn read(&mut self) -> u8 {
        let mut buffer = [0u8; 1];
        match self.file.read(&mut buffer) {
            Ok(_) => buffer[0],
            Err(_) => 0,
        }
    }

    fn write(&mut self, value: u8) {
        self.file.write_all(&[value]).unwrap();
        self.file.flush().unwrap();
    }
}
