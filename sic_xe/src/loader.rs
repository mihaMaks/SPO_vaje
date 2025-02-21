#[derive(Debug)]
pub struct Loader {
    pub start_record: HeaderRecord,
    pub records: Vec<Record>,
    pub end_record: EndRecord,
}

#[derive(Debug)]
pub struct HeaderRecord {
    pub program_name: String,
    pub start_address: u32,
    pub program_length: u32,
}

#[derive(Debug)]
pub enum Record {
    Text(TextRecord),
    // You can add other record types like `D` and `R` if needed.
}

#[derive(Debug)]
pub struct TextRecord {
    pub start_address: u32,
    pub length: u32,
    pub code: Vec<u8>, // Code is stored as Vec<u32> parsed from hexadecimal
}

#[derive(Debug)]
pub struct EndRecord {
    pub start_address: u32,
}
use crate::{Machine, Pomnilnik};
impl Loader {
    /// Creates a new `Loader` instance manually
    pub fn new(start_record: HeaderRecord, records: Vec<Record>, end_record: EndRecord) -> Self {
        Loader {
            start_record,
            records,
            end_record,
        }
    }
    pub fn load_into_memory(&self, machine: &mut Machine) -> Result<(), String> {
        for record in &self.records {
            if let Record::Text(text_record) = record {
                let start_address = text_record.start_address as usize;
                let code = &text_record.code;

                if start_address + code.len() > machine.memory.get_byte_table().len() {
                    return Err(format!(
                        "Record starting at address {:06X} exceeds memory bounds",
                        start_address
                    ));
                }

                for (i, &byte) in code.iter().enumerate() {
                    machine.memory.set_byte(start_address + i, byte)?;
                }
            }
        }
        Ok(())
    }
    fn parse_header(line: &str) -> Option<HeaderRecord> {
        let program_name = line[1..7].trim().to_string();
        let start_address = u32::from_str_radix(&line[7..13], 16).ok()?;
        let program_length = u32::from_str_radix(&line[13..19], 16).ok()?;
        Some(HeaderRecord {
            program_name,
            start_address,
            program_length,
        })
    }
    fn hex_to_u8_vec(hex: &str) -> Vec<u8> {
        hex.as_bytes()
            .chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    fn parse_text(line: &str) -> Option<Record> {
        let start_address = u32::from_str_radix(&line[1..7], 16).ok()?;
        let length = u32::from_str_radix(&line[7..9], 16).ok()?;
        let code = Loader::hex_to_u8_vec(&line[9..]);
        Some(Record::Text(TextRecord {
            start_address,
            length,
            code,
        }))
    }

    fn parse_end(line: &str) -> Option<EndRecord> {
        let start_address = if line.len() > 1 {
            u32::from_str_radix(&line[1..7], 16).ok()?
        } else {
            0
        };
        Some(EndRecord { start_address })
    }

    pub fn parse(content: &str) -> Option<Loader> {
        let mut lines = content.lines();
        let start_record = Self::parse_header(lines.next()?)?;
        let mut records = Vec::new();
        let mut end_record = None;

        for line in lines {
            match line.chars().next()? {
                'T' => {
                    if let Some(record) = Self::parse_text(line) {
                        records.push(record);
                    }
                }
                'E' => {
                    end_record = Self::parse_end(line);
                    break;
                }
                _ => {}
            }
        }

        Some(Loader {
            start_record,
            records,
            end_record: end_record?,
        })
    }
}
