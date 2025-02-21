#[derive(Debug)]
pub struct Pomnilnik {
    pub bit_table: Vec<bool>, // Each element represents a single bit
    byte_table: Vec<u8>,      // Each element represents an unsigned byte (0-255)
}

impl Pomnilnik {
    pub const MAX_ADDRESS: usize = 0x00CFFF; // BFD0 Max address space for SIC/XE (20-bit address space)
    pub fn get_byte_table(&self) -> &Vec<u8> {
        &self.byte_table
    }
    /// Create a new `Pomnilnik` with default values
    pub fn new() -> Self {
        Self {
            bit_table: vec![false; Self::MAX_ADDRESS * 8], // 8 bits per byte
            byte_table: vec![0; Self::MAX_ADDRESS + 1],    // 1 byte per address
        }
    }

    /// Get a byte from the memory at a given address
    pub fn get_byte(&self, addr: Option<usize>) -> Option<u8> {
        // Check if addr is Some and within the maximum address range
        if let Some(address) = addr {
            if address <= Self::MAX_ADDRESS {
                return Some(self.byte_table[address]);
            }
        }
        None // Return None if addr is None or out of bounds
    }

    /// Set a byte in the memory at a given address
    pub fn set_byte(&mut self, addr: usize, value: u8) -> Result<(), String> {
        if addr <= Self::MAX_ADDRESS {
            self.byte_table[addr] = value;
            Ok(())
        } else {
            Err("Address out of bounds".to_string())
        }
    }

    /// Get a word (3 bytes) from the memory at a given address
    pub fn get_word(&self, addr: usize) -> Option<u32> {
        if addr + 2 <= Self::MAX_ADDRESS {
            let word = ((self.byte_table[addr] as u32) << 16)
                | ((self.byte_table[addr + 1] as u32) << 8)
                | (self.byte_table[addr + 2] as u32);
            Some(word)
        } else {
            None // Address out of bounds
        }
    }

    /// Set a word (3 bytes) in the memory at a given address
    pub fn set_word(&mut self, addr: usize, value: u32) -> Result<(), String> {
        if addr + 2 <= Self::MAX_ADDRESS && value <= 0xFFFFFF {
            self.byte_table[addr] = ((value >> 16) & 0xFF) as u8;
            self.byte_table[addr + 1] = ((value >> 8) & 0xFF) as u8;
            self.byte_table[addr + 2] = (value & 0xFF) as u8;
            Ok(())
        } else {
            Err("Address out of bounds or value too large".to_string())
        }
    }

    /// Get a float (6 bytes) from the memory at a given address
    pub fn get_float(&self, addr: usize) -> Option<f64> {
        if addr + 5 <= Self::MAX_ADDRESS {
            let mut bytes = [0u8; 8];
            bytes[2..8].copy_from_slice(&self.byte_table[addr..addr + 6]);
            Some(f64::from_be_bytes(bytes))
        } else {
            None // Address out of bounds
        }
    }

    /// Set a float (6 bytes) in the memory at a given address
    pub fn set_float(&mut self, addr: usize, value: f64) -> Result<(), String> {
        if addr + 5 <= Self::MAX_ADDRESS {
            let bytes = value.to_be_bytes();
            self.byte_table[addr..addr + 6].copy_from_slice(&bytes[2..8]);
            Ok(())
        } else {
            Err("Address out of bounds".to_string())
        }
    }

    /// Get a bit from the bit table at a specific bit address
    pub fn get_bit(&self, bit_addr: usize) -> Option<bool> {
        self.bit_table.get(bit_addr).cloned()
    }

    /// Set a bit in the bit table at a specific bit address
    pub fn set_bit(&mut self, bit_addr: usize, value: bool) -> Result<(), String> {
        if bit_addr < self.bit_table.len() {
            self.bit_table[bit_addr] = value;
            Ok(())
        } else {
            Err("Bit address out of bounds".to_string())
        }
    }

    pub fn print_memory(&self, prog_len: usize) {
        for addr in (0..=prog_len).step_by(16) {
            // Print the address header
            print!("{:06X}: ", addr);

            // Print 16 bytes for the current line
            for offset in 0..16 {
                if addr + offset <= prog_len {
                    print!("{:02X} ", self.byte_table[addr + offset]);
                } else {
                    // Padding for out-of-bounds addresses
                    print!("   ");
                }
            }

            println!(); // Newline for the next row
        }
    }
    pub fn memory_to_string(&self) -> String {
        let mut result = String::new(); // Create a new string buffer

        for addr in (0..=Self::MAX_ADDRESS).step_by(16) {
            // Add the address header to the string
            result.push_str(&format!("{:06X}: ", addr));

            // Add 16 bytes for the current line
            for offset in 0..16 {
                if addr + offset <= Self::MAX_ADDRESS {
                    result.push_str(&format!(" {:02X} ", self.byte_table[addr + offset]));
                } else {
                    // Padding for out-of-bounds addresses
                    result.push_str("   ");
                }
            }
            result.push(' '); // Newline for the next row
            result.push('\n'); // Newline for the next row
        }

        result // Return the accumulated string
    }
    pub fn text_screen(&self) -> String {
        let mut result = String::new(); // Create a new string buffer

        for addr in (0xB800..=0xBFC0).step_by(16) {
            // Add the address header to the string
            result.push_str(&format!("{:06X}: ", addr));

            // Add 16 bytes for the current line
            for offset in 0..16 {
                let charachter: char = self.byte_table[addr + offset] as char;
                if addr + offset <= Self::MAX_ADDRESS && charachter != 0 as char {
                    result.push_str(&format!("{}", charachter));
                } else {
                    // Padding for out-of-bounds addresses
                    result.push_str(" ");
                }
            }

            result.push('\n'); // Newline for the next row
        }

        result // Return the accumulated string
    }
}
