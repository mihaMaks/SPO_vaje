use std::thread;
use std::time::Duration;
mod devices2;
use crate::devices2::StdInputDevice;
use crate::devices2::StdOutputDevice;
use devices2::DeviceManager;
mod pomnilnik;
use devices2::FileDevice;
use gtk::glib::PropertyGet;
use iced::Application;
use opcode::RegisterManager;
use pomnilnik::Pomnilnik;
use std::io::{self, Stdout, Write};
use u12::U12;
mod u12;
mod u24;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use u24::U24;
mod opcode;
use crate::opcode::Opcode;
mod loader;
use crate::loader::Loader;
use crossterm::{
    cursor,
    event::{self, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};
mod gui;
//use loader::{EndRecord, HeaderRecord, Record, TextRecord};
pub struct Machine {
    registers: Vec<u32>, // Registers 0-6, PC (8), and SW (9)
    condition_code: u8,  // Condition code for the SW register
    floating_point: f64, // Register F (optional)
    memory: Pomnilnik,
    device_manager: DeviceManager,
    debug: bool,
    reg_manager: RegisterManager,
    prog_len: usize,
}
impl Machine {
    pub const A: usize = 0; // Register A
    pub const X: usize = 1; // Register X
    pub const L: usize = 2; // Register L
    pub const B: usize = 3; // Register B
    pub const S: usize = 4; // Register S
    pub const T: usize = 5; // Register T
    pub const F: usize = 6; // Register F
    pub const PC: usize = 8; // Program Counter
    pub const SW: usize = 9; // Status Word
    const NUM_DEVICES: usize = 5;
    /// Create a new `Machine` instance with default values for all registers
    pub fn new() -> Self {
        // Standardne naprave
        let mut device_manager = DeviceManager::new();

        // Add standard input and output devices to the manager
        device_manager.add_device("0", Box::new(StdInputDevice::new()));
        device_manager.add_device("1", Box::new(StdOutputDevice::new()));
        Self {
            memory: Pomnilnik::new(),
            registers: vec![0; 10],
            condition_code: 0, // Default condition code is 0x0
            floating_point: 0.0,
            device_manager: device_manager,
            debug: false,
            reg_manager: RegisterManager::new(),
            prog_len: 0,
        }
    }
    pub fn debug(&mut self, b: bool) -> () {
        self.debug = b;
    }
    pub fn set_prog_len(&mut self, prog_len: usize) {
        self.prog_len = prog_len;
    }
    /// Get the value of a register by index
    pub fn get_reg(&self, reg: usize) -> Option<u32> {
        if reg < self.registers.len() {
            Some(self.registers[reg])
        } else {
            None // Return None for invalid indices
        }
    }

    /// Set the value of a register by index
    pub fn set_reg(&mut self, reg: usize, value: u32) {
        if reg < self.registers.len() {
            self.registers[reg] = value;
        } else {
            panic!("Invalid register index: {}", reg);
        }
    }

    /// Get the value of the condition code in SW
    pub fn get_condition_code(&self) -> u8 {
        self.condition_code
    }

    /// Set the condition code in SW
    pub fn set_condition_code(&mut self, value: u8) {
        if value == 0x0 || value == 0x40 || value == 0x80 {
            self.condition_code = value;
        } else {
            panic!("Invalid condition code: {:x}", value);
        }
    }

    /// Get the floating-point register F
    pub fn get_f(&self) -> f64 {
        self.floating_point
    }

    /// Set the floating-point register F
    pub fn set_f(&mut self, value: f64) {
        self.floating_point = value;
    }

    pub fn print_mem(&self) {
        self.memory.print_memory(self.prog_len);
    }

    /// Logs a message for an unimplemented instruction mnemonic.
    pub fn not_implemented(&self, mnemonic: &str) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        writeln!(
            handle,
            "Error: Instruction '{}' is not implemented.",
            mnemonic
        )
        .expect("Failed to write to stderr");
    }

    /// Logs a message for an invalid opcode.
    pub fn invalid_opcode(&self, opcode: usize) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        writeln!(handle, "Error: Invalid opcode encountered: {:#X}.", opcode)
            .expect("Failed to write to stderr");
    }

    /// Logs a message for invalid addressing mode usage.
    pub fn invalid_addressing(&self) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        writeln!(handle, "Error: Invalid addressing mode encountered.")
            .expect("Failed to write to stderr");
    }

    /// Naloži en bajt z naslova v PC, poveča PC za ena in vrne bajt.
    pub fn fetch(&mut self) -> u8 {
        // Preverimo, ali je PC znotraj meja pomnilnika
        if self.registers[Self::PC] as usize >= self.memory.get_byte_table().len() {
            //self.memory.print_memory();
            eprintln!("Error: PC out of memory bounds.");
            panic!(
                "Program Counter (PC) out of bounds! PC={:X}",
                self.registers[Self::PC]
            );
        }

        // Naloži bajt z naslova v PC
        let byte = match self
            .memory
            .get_byte(self.get_reg(Self::PC).map(|val| val as usize))
        {
            Some(value) => value, // Uspešno naložen bajt
            None => {
                eprintln!("Error: Failed to fetch byte from memory.");
                panic!("Fetch failed due to invalid memory address."); // Ali lahko vrnete kakšno privzeto vrednost, če ni panike
            }
        };

        // Poveča PC za ena
        self.registers[Self::PC] += 1;

        // Vrne naloženi bajt
        byte
    }
    pub fn get_rel_addr(&mut self, addres: u16) -> u32 {
        if self.debug {
            println!(
                "pc:{} addres:{}",
                U12::new(self.get_reg(Machine::PC).unwrap() as u16).get(),
                U12::new(addres).get()
            );
        }
        let res = U12::new((addres & 0xFFF))
            .add(U12::new(self.get_reg(Machine::PC).unwrap() as u16))
            .get();
        return res as u32;
    }
    pub fn get_long_addr(&mut self, addres: u16) -> u32 {
        if self.debug {
            println!("get_long_addr!");
        }
        let f4 = self.fetch();
        return (((addres as u32) << 8) + f4 as u32) & 0xFFFFF;
    }
    pub fn set_SW(&mut self, v1: u32, v2: u32) -> () {
        let sw = Machine::SW;
        if v1 < v2 {
            self.set_reg(sw, 0x0);
        }
        if v1 == v2 {
            self.set_reg(sw, 0x40);
        }
        if v1 > v2 {
            self.set_reg(sw, 0x80);
        }
    }
    pub fn ld_reg(
        &mut self,
        reg: usize,
        value: u32,
        naslavljanje: &mut String,
        inst_name: &str,
        print: bool,
    ) -> () {
        self.set_reg(reg, value);
        if print {
            println!(
                "{}({}) reg: {:X}\n",
                inst_name,
                naslavljanje,
                self.get_reg(reg).unwrap(),
            );
        }
    }

    pub fn st_reg(
        &mut self,
        reg_val: u32,
        addres: usize,
        naslavljanje: &str,
        inst_name: &str,
        print: bool,
    ) -> () {
        self.memory.set_word(addres, reg_val);
        if print {
            println!(
                "{}({}) reg={:X} at addres:{:X}\n",
                inst_name, naslavljanje, reg_val, addres
            );
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch(); // Fetch opcode
        if self.execF1(opcode as usize) {
            return;
        } // Try to execute F1
        let operands = self.fetch(); // Fetch operands
        if self.execF2(opcode as usize, operands) {
            return;
        } // Try to execute F2

        let f3 = self.fetch(); // Fetch the next byte for F3/F4

        // Extract the last 2 bits of the opcode for `ni`
        let ni = opcode & 0x03;
        let opcode6 = opcode & 0xFC;

        let addres: u16 = ((operands as u16) << 8) + (f3 as u16);
        if self.debug {
            print!(
                "ukaz={:X} ",
                ((((opcode as u32) << 8) + operands as u32) << 8) + f3 as u32
            );
        }
        if self.execSICF3F4(opcode6 as usize, ni, addres) {
            return;
        } // Execute F3/F4
    }
    pub fn izracunaj_un_ld(
        &mut self,
        ni: u8,
        addres: u16,
        naslavljanje: &mut String,
        long_addr: &mut u32,
    ) -> u32 {
        let xbpe = addres & 0xF000;
        if self.debug {
            print!("ni={} xbpe={:X} addres={:X}\n", ni, xbpe, addres);
        }
        if ni == Opcode::DIRECT && xbpe != Opcode::LONG_ADDR {
            naslavljanje.push_str("op #c");
            return ((addres & 0xFFF) as u32);
        }
        if ni == Opcode::DIRECT && xbpe == Opcode::LONG_ADDR {
            *long_addr = self.get_long_addr(addres);
            naslavljanje.push_str("+op #m");
            return *long_addr;
        }
        let rel_addr = self.get_rel_addr(addres);
        if ni == Opcode::SIMPLE && xbpe == Opcode::PC_REL_ADDR {
            naslavljanje.push_str("op (PC + m)");
            return self.memory.get_word(rel_addr as usize).unwrap();
        }
        if ni == Opcode::SIMPLE && xbpe == Opcode::PC_X_REL_ADDR {
            naslavljanje.push_str("op (PC + X + m)");
            let pc_add_x = U24::new(rel_addr as u64)
                .add(U24::new(self.get_reg(Machine::X).unwrap() as u64))
                .get();
            return self.memory.get_word(pc_add_x as usize).unwrap();
        }

        if ni == Opcode::SIMPLE && xbpe == Opcode::LONG_ADDR {
            naslavljanje.push_str("+op m");

            *long_addr = self.get_long_addr(addres);
            return self.memory.get_word(*long_addr as usize).unwrap();
        }
        if ni == Opcode::SIMPLE && xbpe == Opcode::LONG_ADD_X {
            *long_addr = self.get_long_addr(addres);

            naslavljanje.push_str("+op m,X!");

            return self
                .memory
                .get_word((*long_addr + self.get_reg(Machine::X).unwrap()) as usize)
                .unwrap();
        }
        if ni == Opcode::INDIRECT && xbpe == Opcode::PC_REL_ADDR {
            naslavljanje.push_str("op @m");

            let at_addres = self.memory.get_word(rel_addr as usize).unwrap() as usize;
            if (at_addres as usize) < Pomnilnik::MAX_ADDRESS {
                return self.memory.get_word(at_addres as usize).unwrap();
            } else {
                return 0;
            }
        }
        if ni == Opcode::INDIRECT && xbpe == Opcode::LONG_ADDR {
            *long_addr = self.get_long_addr(addres);
            let at_addres = self.memory.get_word(*long_addr as usize).unwrap() as usize;
            naslavljanje.push_str("+op @m");

            if (at_addres as usize) < Pomnilnik::MAX_ADDRESS {
                return self.memory.get_word(at_addres as usize).unwrap();
            } else {
                return 0;
            }
        }

        return 0;
    }
    pub fn izracunaj_un_st(
        &mut self,
        ni: u8,
        addres: u16,
        naslavljanje: &mut String,
        long_addr: &mut u32,
    ) -> u32 {
        let xbpe = addres & 0xF000;
        if self.debug {
            print!("ni={} xbpe={:X} addres={:X}\n", ni, xbpe, addres);
        }
        if ni == Opcode::DIRECT && xbpe != Opcode::LONG_ADDR {
            naslavljanje.push_str("op #c");
            return (addres as u32) + self.get_reg(Machine::X).unwrap();
        }
        if ni == Opcode::DIRECT && xbpe == Opcode::LONG_ADDR {
            *long_addr = self.get_long_addr(addres);
            naslavljanje.push_str("+op #m");
            return *long_addr;
        }
        let rel_addr = self.get_rel_addr(addres);
        if ni == Opcode::SIMPLE && xbpe == Opcode::PC_REL_ADDR {
            naslavljanje.push_str("op (PC + m)");
            return rel_addr;
        }
        if ni == Opcode::SIMPLE && xbpe == Opcode::PC_X_REL_ADDR {
            naslavljanje.push_str("op (PC + X + m)");
            let pc_add_x = U24::new((addres & 0x000EFF) as u64)
                .add(U24::new(self.get_reg(Machine::X).unwrap() as u64))
                .get();
            return self.memory.get_word(pc_add_x as usize).unwrap();
        }

        if ni == Opcode::SIMPLE && xbpe == Opcode::LONG_ADDR {
            naslavljanje.push_str("+op m");

            *long_addr = self.get_long_addr(addres);
            return self.memory.get_word(*long_addr as usize).unwrap();
        }
        if ni == Opcode::SIMPLE && xbpe == Opcode::LONG_ADD_X {
            *long_addr = self.get_long_addr(addres);

            naslavljanje.push_str("+op m,X!");

            return self
                .memory
                .get_word((*long_addr + self.get_reg(Machine::X).unwrap()) as usize)
                .unwrap();
        }
        if ni == Opcode::INDIRECT && xbpe == Opcode::PC_REL_ADDR {
            naslavljanje.push_str("op @m");

            let at_addres = self.memory.get_word(rel_addr as usize).unwrap() as usize;
            print!("rel:{:X}\n", rel_addr);
            print!("@{:X}\n", at_addres);
            if (at_addres as usize) < Pomnilnik::MAX_ADDRESS {
                return self.memory.get_word(at_addres as usize).unwrap();
            } else {
                return 0;
            }
        }
        if ni == Opcode::INDIRECT && xbpe == Opcode::LONG_ADDR {
            *long_addr = self.get_long_addr(addres);
            let at_addres = self.memory.get_word(*long_addr as usize).unwrap() as usize;
            naslavljanje.push_str("+op @m");

            if (at_addres as usize) < Pomnilnik::MAX_ADDRESS {
                return self.memory.get_word(at_addres as usize).unwrap();
            } else {
                return 0;
            }
        }

        return 0;
    }

    pub fn execF1(&mut self, opcode: usize) -> bool {
        // Match opcode with instructions of format 1
        //println!("Matching F1 instruction");
        match opcode {
            Opcode::FIX => {
                println!("Executing F1 instruction FIX");
                true // Return true if execution was successful
            }
            Opcode::FLOAT => {
                println!("Executing F1 instruction FLOAT");
                true // Return true if execution was successful
            }
            Opcode::NORM => {
                println!("Executing F1 instruction NOMR");
                true // Return true if execution was successful
            }
            // Add more cases for other format 1 opcodes
            _ => false, // Return false if not a valid F1 opcode
        }
    }

    pub fn execF2(&mut self, opcode: usize, operand: u8) -> bool {
        let r1 = (operand >> 4) & 0xF; // Extract the first register
        let r2 = operand & 0xF; // Extract the second register
        if let Some(up1) = self.get_reg(r1 as usize) {
            if let Some(up2) = self.get_reg(r2 as usize) {
                let op1 = U24::new(up1 as u64);
                let op2 = U24::new(up2 as u64);

                match opcode {
                    Opcode::ADDR => {
                        let res = op1.add(op2).get();
                        self.set_reg(r2 as usize, res);

                        println!(
                            "ADDR  {:?}={:X} + {:?}={:X} => {:?}={:X}",
                            self.reg_manager.reg_name.get(&(r1 as usize)),
                            op1.get(),
                            self.reg_manager.reg_name.get(&(r2 as usize)),
                            op2.get(),
                            self.reg_manager.reg_name.get(&(r2 as usize)),
                            res
                        );
                        true
                    }
                    Opcode::CLEAR => {
                        self.set_reg(r1 as usize, r2 as u32);
                        println!("CLEAR {:?}", self.reg_manager.reg_name.get(&(r1 as usize)));
                        true
                    }
                    Opcode::COMPR => {
                        self.set_SW(op1.get(), op2.get());

                        println!("COMPR {} : {}", op1.get(), op2.get());
                        true
                    }
                    Opcode::DIVR => {
                        let res = op1.div(op2).get();
                        self.set_reg(r2 as usize, res);
                        println!(
                            "DIVR with {} / {} = {:?}={}",
                            op2.get(),
                            op1.get(),
                            self.reg_manager.reg_name.get(&(r2 as usize)),
                            res
                        );
                        true // Return true if execution was successful
                    }
                    Opcode::MULR => {
                        let res = op1.mul(op2).get();
                        self.set_reg(r2 as usize, res);
                        println!(
                            "MULR {} * {} = {:?}={}",
                            op1.get(),
                            op2.get(),
                            self.reg_manager.reg_name.get(&(r2 as usize)),
                            res
                        );
                        true // Return true if execution was successful
                    }
                    Opcode::RMO => {
                        self.set_reg(r2 as usize, op1.get());
                        println!(
                            "RMO {:?} => {:?}\n{} => {:?}",
                            self.reg_manager.reg_name.get(&(r1 as usize)),
                            self.reg_manager.reg_name.get(&(r2 as usize)),
                            op1.get(),
                            self.reg_manager.reg_name.get(&(r2 as usize))
                        );
                        true // Return true if execution was successful
                    }
                    Opcode::SHIFTL => {
                        self.set_reg(r1 as usize, op1.get() << (r2 + 1));

                        println!("SHIFTL {} << {}", op1.get(), (r2 + 1));
                        true // Return true if execution was successful
                    }
                    Opcode::SHIFTR => {
                        // Example for ADDR (opcode 0x24)
                        self.set_reg(r1 as usize, op1.get() >> (r2 + 1));

                        println!("SHIFTL {} << {}", op1.get(), (r2 + 1));
                        true // Return true if execution was successful
                    }
                    Opcode::SUBR => {
                        let res = op2.sub(op1).get();
                        self.set_reg(r2 as usize, res);
                        println!(
                            "SUBR {:?} - {} = {:?}={}",
                            op1.get(),
                            op2.get(),
                            self.reg_manager.reg_name.get(&(r2 as usize)),
                            res
                        );
                        true
                    }
                    Opcode::TIXR => {
                        let x_val = op2.get() + 1;
                        self.set_reg(r2 as usize, x_val);
                        self.set_SW(x_val, op1.get());

                        println!("TIXR X={} : {}", x_val, r1);
                        true
                    }
                    _ => false, // Return false if not a valid F2 opcode
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    // #######################################################################
    // ###########################  F 3/4   ##################################
    // #######################################################################
    pub fn execSICF3F4(&mut self, opcode: usize, ni: u8, mut addres: u16) -> bool {
        let mut naslavljanje = String::new();
        let mut long_addr: u32 = 0;

        let mut un = self.izracunaj_un_st(ni, addres, &mut naslavljanje, &mut long_addr);
        if self.debug {
            print!("un: {:X}\n", un);
        }

        match opcode {
            Opcode::ADD => {
                let reg = Machine::A;
                un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                let a = U24::new(self.get_reg(reg).unwrap() as u64);
                self.set_reg(reg, a.add(U24::new(un as u64)).get());
                print!(
                    "ADD {:X} + {:X} = {:X}\n",
                    a.get(),
                    un,
                    self.get_reg(reg).unwrap()
                );

                return true; // Return true if execution was successful
            }
            Opcode::SUB => {
                let reg = Machine::A;
                un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                let a = U24::new(self.get_reg(reg).unwrap() as u64);
                self.set_reg(reg, a.sub(U24::new(un as u64)).get());
                print!(
                    "SUB {:X} + {:X} = {:X}\n",
                    a.get(),
                    un,
                    self.get_reg(reg).unwrap()
                );

                return true;
            }
            Opcode::AND => {
                let reg = Machine::A;
                let a = U24::new(self.get_reg(reg).unwrap() as u64);
                let new_a = a.get() & un;
                self.set_reg(reg, new_a);

                println!("AND A={:X} : {:X} = {:X}\n", a.get(), un, new_a);
                return true;
            }
            opcode::Opcode::COMP => {
                let reg = Machine::A;
                let v1 = self.get_reg(reg as usize).unwrap();
                let v2 = un;
                self.set_SW(v1, v2);
                println!(
                    "COMP A={:X} : {:X} => {:X}\n",
                    v1,
                    v2,
                    self.get_reg(Machine::SW).unwrap()
                );
                return true;
            }

            Opcode::DIV => {
                let reg = Machine::A;
                un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                let a = U24::new(self.get_reg(reg).unwrap() as u64);
                let new_a = a.div(U24::new(un as u64)).get();
                self.set_reg(reg, new_a);
                println!("DIV A={} - {} = {}\n", a.get(), un, new_a);
                return true;
            }
            Opcode::MUL => {
                let reg = Machine::A;
                un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                let a = U24::new(self.get_reg(reg).unwrap() as u64);
                self.set_reg(reg, a.mul(U24::new(un as u64)).get());
                print!(
                    "MUL {:X} * {:X} = {:X}\n",
                    a.get(),
                    un,
                    self.get_reg(reg).unwrap()
                );
                return true;
            }
            Opcode::J => {
                let pc = Machine::PC;
                let pe = addres & 0x3000;
                if ni == Opcode::SIMPLE && pe == Opcode::PC_REL_ADDR {
                    let rel_addr = self.get_rel_addr(addres);
                    self.set_reg(pc, rel_addr);
                    println!("JUMP PC={}\n", rel_addr);
                } else if ni == Opcode::SIMPLE && pe == Opcode::LONG_ADDR {
                    self.set_reg(pc, long_addr);
                    println!("JUMP PC={}\n", long_addr);
                } else {
                    println!("THIS KING OF ADDRESING NOT IMPLEMENTED\n");
                }
                return true;
            }
            Opcode::JEQ => {
                let pc = Machine::PC;
                let sw = Machine::SW;

                let pe = addres & 0x3000;
                if ni == Opcode::SIMPLE && pe == Opcode::PC_REL_ADDR {
                    let rel_addr = self.get_rel_addr(addres);
                    if self.get_reg(sw).unwrap() == 0x40 {
                        self.set_reg(pc, rel_addr);
                    }
                    println!("JEQ PC={}\n", rel_addr);
                } else if ni == Opcode::SIMPLE && pe == Opcode::LONG_ADDR {
                    if self.get_reg(sw).unwrap() == 0x40 {
                        self.set_reg(pc, long_addr);
                    }
                    println!("JEQ PC={}\n", long_addr);
                } else {
                    println!("THIS KING OF ADDRESING NOT IMPLEMENTED\n");
                }
                return true;
            }
            Opcode::JGT => {
                let pc = Machine::PC;
                let sw = Machine::SW;

                let pe = addres & 0x3000;
                if ni == Opcode::SIMPLE && pe == Opcode::PC_REL_ADDR {
                    let rel_addr = self.get_rel_addr(addres);
                    if self.get_reg(sw).unwrap() == 0x80 {
                        self.set_reg(pc, rel_addr);
                    }
                    println!("JGT PC={}\n", rel_addr);
                } else if ni == Opcode::SIMPLE && pe == Opcode::LONG_ADDR {
                    if self.get_reg(sw).unwrap() == 0x80 {
                        self.set_reg(pc, long_addr);
                    }
                    println!("JGT PC={}\n", long_addr);
                } else {
                    println!("THIS KING OF ADDRESING NOT IMPLEMENTED\n");
                }
                return true;
            }
            Opcode::JLT => {
                let pc = Machine::PC;
                let sw = Machine::SW;

                let pe = addres & 0x3000;
                if ni == Opcode::SIMPLE && pe == Opcode::PC_REL_ADDR {
                    let rel_addr = self.get_rel_addr(addres);
                    if self.get_reg(sw).unwrap() == 0x00 {
                        self.set_reg(pc, rel_addr);
                    }
                    println!("JLT PC={}\n", rel_addr);
                } else if ni == Opcode::SIMPLE && pe == Opcode::LONG_ADDR {
                    if self.get_reg(sw).unwrap() == 0x00 {
                        self.set_reg(pc, long_addr);
                    }
                    println!("JLT PC={}\n", long_addr);
                } else {
                    println!("THIS KING OF ADDRESING NOT IMPLEMENTED\n");
                }
                return true;
            }
            Opcode::LDA => {
                let reg = Machine::A;
                un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                self.ld_reg(reg, un, &mut naslavljanje, Opcode::LDA_STR, true);
                //self.ld_instruction(ni, addres, reg, Opcode::LDA_STR);

                return true;
            }
            Opcode::LDB => {
                let reg = Machine::B;
                print!("LDB\n");
                let un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                self.ld_reg(reg, un, &mut naslavljanje, Opcode::LDB_STR, true);

                return true;
            }
            Opcode::LDCH => {
                let reg = Machine::A;
                let value = ((un & 0xFF0000) >> 16);
                let a = self.get_reg(reg).unwrap();
                let new_a = (a & 0xFFFF00) | value;
                if self.debug {
                    self.ld_reg(reg, new_a as u32, &mut naslavljanje, Opcode::LDCH_STR, true);
                }
                return true;
            }
            Opcode::LDL => {
                let reg = Machine::L;
                let un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                self.ld_reg(reg, un, &mut naslavljanje, Opcode::LDB_STR, true);

                return true;
            }
            Opcode::LDS => {
                let reg = Machine::S;
                let un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                self.ld_reg(reg, un, &mut naslavljanje, Opcode::LDS_STR, true);

                return true;
            }
            Opcode::LDT => {
                let reg = Machine::T;
                let un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                self.ld_reg(reg, un, &mut naslavljanje, Opcode::LDT_STR, true);

                return true;
            }
            Opcode::LDX => {
                let reg = Machine::X;
                let un = self.izracunaj_un_ld(ni, addres, &mut naslavljanje, &mut long_addr);
                self.ld_reg(reg, un, &mut naslavljanje, Opcode::LDX_STR, true);

                return true;
            }

            Opcode::OR => {
                let reg = Machine::A;
                let a = U24::new(self.get_reg(reg).unwrap() as u64);
                let new_a = a.get() | un;
                self.set_reg(reg, new_a);

                println!("OR A={:X} : {:X} = {:X}\n", a.get(), un, new_a);
                return true;
            }
            Opcode::TIX => {
                let reg = Machine::X;
                self.set_reg(reg, self.get_reg(reg).unwrap() + 1);
                let v1 = self.get_reg(reg).unwrap();
                let v2 = un;
                self.set_SW(v1, v2);
                println!(
                    "TIX X={:X} : {:X} PC={}\n",
                    v1,
                    v2,
                    self.get_reg(Machine::PC).unwrap()
                );

                return true;
            }
            Opcode::JSUB => {
                let pc = Machine::PC;
                let l = Machine::L;

                let pe = addres & 0x7000;
                if ni == Opcode::SIMPLE && pe == Opcode::PC_REL_ADDR {
                    self.set_reg(l, self.get_reg(pc).unwrap()); // L <- (PC)
                    let rel_addr = self.get_rel_addr(addres); // PC <- m
                    self.set_reg(pc, rel_addr);
                    println!("{} PC={}\n", Opcode::JSUB_STR, self.get_reg(pc).unwrap());
                } else if ni == Opcode::SIMPLE && pe == Opcode::LONG_ADDR {
                    self.set_reg(l, self.get_reg(pc).unwrap()); // L <- (PC)
                    self.set_reg(pc, long_addr);
                    println!("{} PC={}\n", Opcode::JSUB_STR, self.get_reg(pc).unwrap());
                } else {
                    println!("THIS KING OF ADDRESING NOT IMPLEMENTED\n");
                }
                return true;
            }
            Opcode::RSUB => {
                self.set_reg(Machine::PC, self.get_reg(Machine::L).unwrap());
                println!("RSUB");
                return true;
            }
            Opcode::STA => {
                let reg = Machine::A;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STA_STR,
                    true,
                );
                return true;
            }
            Opcode::STB => {
                let reg = Machine::B;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STB_STR,
                    true,
                );

                return true;
            }
            Opcode::STX => {
                let reg = Machine::X;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STX_STR,
                    true,
                );

                return true;
            }
            Opcode::STL => {
                let reg = Machine::L;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STL_STR,
                    true,
                );

                return true;
            }
            Opcode::STT => {
                let reg = Machine::T;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STT_STR,
                    true,
                );

                return true;
            }
            Opcode::STSW => {
                let reg = Machine::SW;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STSW_STR,
                    true,
                );

                return true;
            }
            Opcode::STS => {
                let reg = Machine::S;
                let reg_val = self.get_reg(reg).unwrap();
                self.st_reg(
                    reg_val,
                    un as usize,
                    &mut naslavljanje,
                    Opcode::STS_STR,
                    true,
                );

                return true;
            }
            Opcode::STCH => {
                let a = Machine::A;
                let val_a = self.get_reg(a).unwrap();
                let res = (val_a & 0x0000FF) as u8;
                let xbpe = addres & 0xF000;
                if ni == Opcode::DIRECT && xbpe != Opcode::LONG_ADDR {
                    println!("STCH ch={:X} at={:X}", res, addres);
                    self.memory.set_byte(addres as usize, res);
                }
                if ni == Opcode::DIRECT && xbpe == Opcode::LONG_ADDR {
                    println!("STCH ch={:X} at={:X}", res, long_addr);

                    self.memory.set_byte(long_addr as usize, res);
                }
                let rel_addr = self.get_rel_addr(addres) as usize;
                if ni == Opcode::SIMPLE && xbpe == Opcode::PC_REL_ADDR {
                    println!("STCH ch={:X} at={:X}", res, rel_addr);

                    self.memory.set_byte(rel_addr, res);
                }
                if ni == Opcode::SIMPLE && xbpe == Opcode::PC_X_REL_ADDR {
                    let mut pc_add_x = (un as usize) + self.get_reg(Machine::X).unwrap() as usize;
                    //print!("{:X} ", un);
                    pc_add_x = U24::new((addres & 0x000EFF) as u64)
                        .add(U24::new(self.get_reg(Machine::X).unwrap() as u64))
                        .get() as usize;
                    println!("STCH ch={:X} at={:X}", res, pc_add_x);

                    self.memory.set_byte(pc_add_x, res);
                }
                if ni == Opcode::SIMPLE && xbpe == Opcode::LONG_ADDR {
                    println!("STCH ch={:X} at={:X}", res, long_addr);

                    self.memory.set_byte(long_addr as usize, res);
                }
                if ni == Opcode::SIMPLE && xbpe == Opcode::LONG_ADD_X {
                    let long_addr_add_x = long_addr + self.get_reg(Machine::X).unwrap();
                    println!("STCH ch={:X} at={:X}", res, long_addr_add_x);

                    self.memory.set_byte(long_addr_add_x as usize, res);
                }
                if ni == Opcode::INDIRECT && xbpe == Opcode::PC_REL_ADDR {
                    let at_addres = self.memory.get_word(rel_addr).unwrap() as usize;
                    println!("STCH ch={:X} at={:X}", res, at_addres);

                    self.memory.set_byte(at_addres, res);
                }
                if ni == Opcode::INDIRECT && xbpe == Opcode::LONG_ADDR {
                    let at_addres = self.memory.get_word(long_addr as usize).unwrap() as usize;
                    println!("STCH ch={:X} at={:X}", res, at_addres);

                    self.memory.set_byte(at_addres, res);
                }
                return true;
            }
            Opcode::WD => {
                let reg = Machine::A;
                let out = self.get_reg(reg).unwrap() as u8;
                println!("WRITE DEVICE:{} | A={}\n", un, out as char);
                if let Some(device) = self.device_manager.get_device(&un.to_string()) {
                    device.write_byte(out);
                } else {
                    self.device_manager
                        .add_device(&un.to_string(), Box::new(FileDevice::new(&un.to_string())));
                    if let Some(device) = self.device_manager.get_device(&un.to_string()) {
                        device.write_byte(out);
                    } else {
                        println!("Couldnt write to device: {}", un);
                    }
                }
                return true;
            }
            Opcode::RD => {
                let reg = Machine::A;
                println!("READ DEVICE:{}", un);
                if un == 0 {
                    println!("Listening on stdin..");
                    let device = self.device_manager.get_device(&un.to_string()).unwrap();
                    if let Some(b) = device.read_byte() {
                        let a = self.get_reg(reg).unwrap() & 0xFFFF00;
                        self.set_reg(reg, a | (b as u32));
                        if self.debug {
                            println!("in reg a: {:X}", self.get_reg(reg).unwrap());
                        }
                    } else {
                        println!("Couldnt read from device {}", un);
                    }
                } else if let Some(device) = self.device_manager.get_device(&un.to_string()) {
                    if let Some(b) = device.read_byte() {
                        let a = self.get_reg(reg).unwrap() & 0xFFFF00;
                        self.set_reg(reg, a | (b as u32));
                        if self.debug {
                            println!("in reg a: {:X}", self.get_reg(reg).unwrap());
                        }
                    } else {
                        self.set_reg(reg, 0);
                        //println!("1Couldnt read from device {}", un);
                    }
                } else {
                    self.device_manager
                        .add_device(&un.to_string(), Box::new(FileDevice::new(&un.to_string())));
                    if let Some(device) = self.device_manager.get_device(&un.to_string()) {
                        if let Some(b) = device.read_byte() {
                            let a = self.get_reg(reg).unwrap() & 0xFFFF00;
                            self.set_reg(reg, a | (b as u32));
                        } else {
                            println!("2Couldnt read from device {}", un);
                        }
                    } else {
                        println!("Couldnt get device: {}", un);
                    }
                }
                return true;
            }

            _ => {
                return false;
            }
        }
    }
    pub fn write_terminal(&mut self) -> io::Result<()> {
        //terminal::enable_raw_mode()?;

        let mut stdout = io::stdout();

        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // Custom string (could be dynamically updated in each iteration)
        let frame = self.memory.text_screen();

        // Print the custom string
        println!("{}", frame);

        // Flush the output to the terminal
        stdout.flush()?;
        //terminal::disable_raw_mode()?;

        Ok(())
    }
    pub fn run_auto(&mut self, screen: &str, wait_time: u64) -> String {
        // Shared state to indicate if we should stop the loop
        let stop_flag = Arc::new(Mutex::new(false));

        // Loop to animate

        // Clone the Arc to pass to the input handling thread
        let stop_flag_clone = Arc::clone(&stop_flag);

        // Spawn a thread to listen for stdin input
        thread::spawn(move || {
            let mut input = String::new();
            loop {
                // Read the input from stdin
                if io::stdin().read_line(&mut input).is_ok() && !input.is_empty() {
                    // Set stop_flag to true if input is received
                    let mut flag = stop_flag_clone.lock().unwrap();
                    *flag = true;
                    break;
                }
                input.clear(); // Clear the input buffer after each check
            }
        });

        println!("The loop will run every second unless input is received.");

        let timeout = Duration::from_secs(wait_time); // Loop every second
        while !*stop_flag.lock().unwrap() {
            // The loop runs every second
            //println!("Looping...");
            thread::sleep(timeout);

            if screen == "true" {
                self.execute();
                self.write_terminal().expect("REASON");
            } else {
                self.execute();
            }
        }

        println!("Input received, loop stopped.");
        println!("For resuming press Enter");
        println!("For step by step mode write step");

        // Now we need to handle the input to determine the mode
        let mut mode = String::new(); // Initialize the mode as an empty string
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        // Clean up the input
        if mode.trim() == "" {
            mode.clear(); // If input is just a newline, clear the mode
            mode.push_str("auto"); // Set mode to "auto"
        } else if mode.trim() == "step" {
            mode.clear(); // If "step" is entered, clear mode and set it to "step"
            mode.push_str("step");
        } else {
            mode.trim(); // Trimming any extra spaces, but not modifying `mode`
        }

        // Return the `mode` string
        mode
    }
    pub fn run_step(&mut self, screen: &str) -> String {
        println!(
            "Step by step mode. For step pres Enter. To stop execturin press ^C\nOr write stop"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        while input != "stop\n" {
            //print!("Read: {}", input);
            if input == "memo\n" {
                self.print_mem();
            } else if input == "text\n" {
                print!("{}", self.memory.text_screen());
            } else if screen == "true" {
                self.execute();
                println!("{}", self.memory.text_screen());
            } else {
                self.execute();
            }
            // to do if screen == text
            input = "".to_string();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }
        println!("loop stopped.");
        println!("For resuming press Enter");
        println!("For auto mode write auto");
        let mut mode = String::new(); // Initialize the mode as an empty string
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        // Clean up the input
        if mode.trim() == "" {
            mode.clear(); // If input is just a newline, clear the mode
            mode.push_str("step"); // Set mode to "auto"
        } else if mode.trim() == "auto" {
            mode.clear(); // If "step" is entered, clear mode and set it to "step"
            mode.push_str("auto");
        } else {
            mode.trim(); // Trimming any extra spaces, but not modifying `mode`
        }

        // Return the `mode` string
        mode
    }

    pub fn run(&mut self, mode_str: &str, screen: &str, hz_str: &str) -> () {
        let hz: u64 = hz_str.parse().unwrap();
        let wait_time = 1 / hz;
        let mut mode = mode_str.to_string();

        while true {
            while mode == "auto" {
                mode.clear();
                mode.push_str(&self.run_auto(screen, wait_time));
            }

            while mode == "step" {
                mode.clear();
                mode.push_str(&self.run_step(screen));
            }
            //thread::sleep(Duration::from_secs(1));
        }
    }
}
use std::fs::File;
use std::io::Read;
fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Create a string to hold the contents
    let mut contents = String::new();

    // Read the file contents into the string
    file.read_to_string(&mut contents)?;

    // Return the contents
    Ok(contents)
}

use crate::gui::App;
use std::env;
use std::sync::{Arc, Mutex};
fn main() {
    //          mode debug text_screen frequency
    // cargo run auto false false 1 0 ../dn1/obj/ftf.obj
    // ###################### LOADER #############################
    let args: Vec<String> = env::args().collect();
    if (args[5] == "1") {
        App::run(iced::Settings {
            flags: args[6].clone(),
            ..iced::Settings::default()
        });
    }
    // let mut filepath = "./../SPO-Asm/src/sic/my_arith.obj";
    //let mut filepath = "./../SPO-Asm/src/sic/my_ball.obj";
    //let filepath = "./obj/ball.obj";
    let mut filepath = &args[6];

    //filepath = &args[5];
    match read_file_to_string(&filepath) {
        Ok(obj_file) => {
            if let Some(loader) = Loader::parse(&obj_file) {
                let mut machine = Machine::new();
                // Load program
                loader.load_into_memory(&mut machine);
                // Set PC to start addres
                machine.set_reg(Machine::PC, loader.start_record.start_address);
                // ######## GUI ###########

                // Lengath of program
                let prog_len = loader.start_record.program_length;
                machine.set_prog_len(prog_len as usize);

                println!("Length of programm: {} bytes", prog_len);
                //println!("Running program: {}", args[0]);
                //println!("# args:{}", args.len());
                if args.len() == 7 {
                    println!(
                        "mode: {}, debug: {}, screen: {}, Hz: {}",
                        &args[1], &args[2], &args[3], &args[4]
                    );
                    if &args[2] == "true" {
                        machine.debug(true);
                    }
                    machine.run(&args[1], &args[3], &args[4]);
                } else {
                    for arg in &args {
                        println!("{}", arg);
                    }
                    println!("args len is {}", args.len());
                    println!(
                        "Provide argument \n arg[3]: 'auto' or 'step'\n
                            arg[4]: true/false\n arg[5]: 1-1000"
                    );
                }
            } else {
                println!("Failed to parse .obj file.");
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
