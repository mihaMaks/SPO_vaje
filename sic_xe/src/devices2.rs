use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout};
use std::io::{Read, Write};

pub trait Device {
    /// Reads a single byte from the device.
    fn read_byte(&mut self) -> Option<u8>;

    /// Writes a single byte to the device.
    fn write_byte(&mut self, byte: u8);
}

// InputDevice using a generic Read source
pub struct InputDevice {
    input: Box<dyn Read>,
}

impl InputDevice {
    pub fn new(input: Box<dyn Read>) -> Self {
        InputDevice { input }
    }
}

impl Device for InputDevice {
    fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0u8; 1]; // Buffer for a single byte
        self.input.read_exact(&mut buffer).ok()?; // Read exactly 1 byte
        Some(buffer[0])
    }

    fn write_byte(&mut self, _byte: u8) {
        println!("InputDevice: Write not supported");
    }
}

// OutputDevice using a generic Write destination
pub struct OutputDevice {
    output: Box<dyn Write>,
}

impl OutputDevice {
    pub fn new(output: Box<dyn Write>) -> Self {
        OutputDevice { output }
    }
}

impl Device for OutputDevice {
    fn read_byte(&mut self) -> Option<u8> {
        println!("OutputDevice: Read not supported");
        None
    }

    fn write_byte(&mut self, byte: u8) {
        self.output.write_all(&[byte]).unwrap(); // Write a single byte
        self.output.flush().unwrap();
    }
}

// FileDevice for reading and writing to files
pub struct FileDevice {
    file: File,
}

impl FileDevice {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        FileDevice { file }
    }
}

impl Device for FileDevice {
    fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0u8; 1]; // Buffer for a single byte
        self.file.read_exact(&mut buffer).ok()?; // Read exactly 1 byte
        Some(buffer[0])
    }

    fn write_byte(&mut self, byte: u8) {
        self.file.write_all(&[byte]).unwrap(); // Write a single byte
        self.file.flush().unwrap();
    }
}

// Standard input device that reads from stdin
pub struct StdInputDevice;

impl StdInputDevice {
    pub fn new() -> Self {
        StdInputDevice
    }
}

impl Device for StdInputDevice {
    fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0u8; 1]; // Buffer for a single byte
        stdin().read_exact(&mut buffer).ok()?; // Read exactly 1 byte from stdin
        Some(buffer[0])
    }

    fn write_byte(&mut self, _byte: u8) {
        println!("StdInputDevice: Write not supported");
    }
}

// Standard output device that writes to stdout
pub struct StdOutputDevice;

impl StdOutputDevice {
    pub fn new() -> Self {
        StdOutputDevice
    }
}

impl Device for StdOutputDevice {
    fn read_byte(&mut self) -> Option<u8> {
        println!("StdOutputDevice: Read not supported");
        None
    }

    fn write_byte(&mut self, byte: u8) {
        stdout().write_all(&[byte]).unwrap(); // Write a single byte to stdout
        stdout().flush().unwrap();
    }
}

// DeviceManager that manages devices
pub struct DeviceManager {
    devices: HashMap<String, Box<dyn Device>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, name: &str, device: Box<dyn Device>) {
        self.devices.insert(name.to_string(), device);
    }

    pub fn get_device(&mut self, name: &str) -> Option<&mut Box<dyn Device>> {
        self.devices.get_mut(name)
    }
}
/*
fn main() {
    let mut device_manager = DeviceManager::new();

    // Add standard input and output devices to the manager
    device_manager.add_device("stdin", Box::new(StdInputDevice::new()));
    device_manager.add_device("stdout", Box::new(StdOutputDevice::new()));

    // Add other devices (file, custom input, etc.) as needed
    // Example: device_manager.add_device("file_device", Box::new(FileDevice::new("file.txt")));

    // Example usage
    let mut stdout_device = device_manager.get_device("stdout").unwrap();
    stdout_device.write_byte(b'H');
    stdout_device.write_byte(b'e');
    stdout_device.write_byte(b'l');
    stdout_device.write_byte(b'l');
    stdout_device.write_byte(b'o');
}

*/
