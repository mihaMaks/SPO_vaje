pub struct Opcode;

impl Opcode {
    pub const LONG_ADDR: u16 = 4096;
    pub const PC_REL_ADDR: u16 = 0x2000;
    pub const PC_X_REL_ADDR: u16 = 0x8000;
    pub const LONG_ADD_X: u16 = 36864;
    pub const SIMPLE: u8 = 3;
    pub const DIRECT: u8 = 1;
    pub const INDIRECT: u8 = 2;

    pub const ADD: usize = 0x18; // A <-- (A) + (m..m+2)
    pub const ADD_TYPE: &str = "3/4";
    pub const ADD_STR: &str = "ADD";

    pub const SUB: usize = 0x1C; // A <-- (A) + (m..m+2)
    pub const SUB_TYPE: &str = "3/4";
    pub const SUB_STR: &str = "SUB";

    pub const ADDF: usize = 0x58; // F <-- (F) + (m..m+5)
    pub const ADDF_TYPE: &str = "3/4";
    pub const ADDF_STR: &str = "ADDF";

    pub const ADDR: usize = 0x90; // r2 <-- (r2) + (r1)
    pub const ADDR_TYPE: &str = "2";
    pub const ADDR_STR: &str = "ADDR";

    pub const AND: usize = 0x40; // A <-- (A) & (m..m+2)
    pub const AND_TYPE: &str = "3/4";
    pub const AND_STR: &str = "AND";

    pub const CLEAR: usize = 0xB4; // r1 <-- 0
    pub const CLEAR_TYPE: &str = "2";
    pub const CLEAR_STR: &str = "CLEAR";

    pub const COMP: usize = 0x28; // Compare A : (m..m+2)
    pub const COMP_TYPE: &str = "3/4";
    pub const COMP_STR: &str = "COMP";

    pub const COMPF: usize = 0x88; // Compare F : (m..m+5)
    pub const COMPF_TYPE: &str = "3/4";
    pub const COMPF_STR: &str = "COMPF";

    pub const COMPR: usize = 0xA0; // Compare (r1) : (r2)
    pub const COMPR_TYPE: &str = "2";
    pub const COMPR_STR: &str = "COMPR";

    pub const DIV: usize = 0x24; // A <-- (A) / (m..m+2)
    pub const DIV_TYPE: &str = "3/4";
    pub const DIV_STR: &str = "DIV";

    pub const DIVF: usize = 0x64; // F <-- (F) / (m..m+5)
    pub const DIVF_TYPE: &str = "3/4";
    pub const DIVF_STR: &str = "DIVF";

    pub const DIVR: usize = 0x9C; // r2 <-- (r2) / (r1)
    pub const DIVR_TYPE: &str = "2";
    pub const DIVR_STR: &str = "DIVR";

    pub const FIX: usize = 0xC4; // A <-- (F) [convert to integer]
    pub const FIX_TYPE: &str = "1";
    pub const FIX_STR: &str = "FIX";

    pub const FLOAT: usize = 0xC0; // F <-- (A) [convert to floating]
    pub const FLOAT_TYPE: &str = "1";
    pub const FLOAT_STR: &str = "FLOAT";

    pub const HIO: usize = 0xF4; // Halt I/O
    pub const HIO_TYPE: &str = "1";
    pub const HIO_STR: &str = "HIO";

    pub const J: usize = 0x3C; // PC <-- m
    pub const J_TYPE: &str = "3/4";
    pub const J_STR: &str = "J";

    pub const JEQ: usize = 0x30; // PC <-- m if CC = 0
    pub const JEQ_TYPE: &str = "3/4";
    pub const JEQ_STR: &str = "JEQ";

    pub const JGT: usize = 0x34; // PC <-- m if CC > 0
    pub const JGT_TYPE: &str = "3/4";
    pub const JGT_STR: &str = "JGT";

    pub const JLT: usize = 0x38; // PC <-- m if CC < 0
    pub const JLT_TYPE: &str = "3/4";
    pub const JLT_STR: &str = "JLT";

    pub const JSUB: usize = 0x48; // L <-- (PC); PC <-- m
    pub const JSUB_TYPE: &str = "3/4";
    pub const JSUB_STR: &str = "JSUB";

    pub const LDA: usize = 0x00; // A <-- (m..m+2)
    pub const LDA_TYPE: &str = "3/4";
    pub const LDA_STR: &str = "LDA";

    pub const LDB: usize = 0x68; // B <-- (m..m+2)
    pub const LDB_TYPE: &str = "3/4";
    pub const LDB_STR: &str = "LDB";

    pub const LDCH: usize = 0x50; // A [rightmost byte] <-- (m)
    pub const LDCH_TYPE: &str = "3/4";
    pub const LDCH_STR: &str = "LDCH";

    pub const LDF: usize = 0x70; // F <-- (m..m+5)
    pub const LDF_TYPE: &str = "3/4";
    pub const LDF_STR: &str = "LDF";

    pub const LDL: usize = 0x08; // L <-- (m..m+2)
    pub const LDL_TYPE: &str = "3/4";
    pub const LDL_STR: &str = "LDL";

    pub const LDS: usize = 0x6C; // S <-- (m..m+2)
    pub const LDS_TYPE: &str = "3/4";
    pub const LDS_STR: &str = "LDS";

    pub const LDT: usize = 0x74; // T <-- (m..m+2)
    pub const LDT_TYPE: &str = "3/4";
    pub const LDT_STR: &str = "LDT";

    pub const LDX: usize = 0x04; // X <-- (m..m+2)
    pub const LDX_TYPE: &str = "3/4";
    pub const LDX_STR: &str = "LDX";

    pub const SHIFTL: usize = 0xA4; // r1 <-- (r1); left circular shift
    pub const SHIFTL_TYPE: &str = "2";
    pub const SHIFTL_STR: &str = "SHIFTL";

    pub const SHIFTR: usize = 0xA8; // r1 <-- (r1); right circular shift
    pub const SHIFTR_TYPE: &str = "2";
    pub const SHIFTR_STR: &str = "SHIFTR";

    pub const SUBR: usize = 0x94; // r2 <-- (r2) - (r1)
    pub const SUBR_TYPE: &str = "2";
    pub const SUBR_STR: &str = "SUBR";

    pub const OR: usize = 0x44; // A <-- (A) | (m..m+2)
    pub const OR_TYPE: &str = "3/4";
    pub const OR_STR: &str = "OR";

    pub const MULR: usize = 0x98; // r2 <-- (r2) * (r1)
    pub const MULR_TYPE: &str = "2";
    pub const MULR_STR: &str = "MULR";

    pub const RMO: usize = 0xAC; // r2 <-- (r1)
    pub const RMO_TYPE: &str = "2";
    pub const RMO_STR: &str = "RMO";

    pub const MUL: usize = 0x20; // A <-- (A) * (m..m+2)
    pub const MUL_TYPE: &str = "3/4";
    pub const MUL_STR: &str = "MUL";

    pub const TIX: usize = 0x2C; // X <-- (X) + 1; (X) : (m..m+2)
    pub const TIX_TYPE: &'static str = "3/4";
    pub const TIX_STR: &str = "TIX";

    pub const TIXR: usize = 0xB8; // X <-- (X
    pub const TIXR_TYPE: &'static str = "2";
    pub const TIXR_STR: &str = "TIXR";

    pub const RSUB: usize = 0x4C; // PC <-- (L)
    pub const RSUB_TYPE: &str = "3/4";
    pub const RSUB_STR: &str = "RSUB";

    pub const NORM: usize = 0xC8; // F <-- (F) [normalized]
    pub const NORM_TYPE: &str = "1";
    pub const NORM_STR: &str = "NORM";

    pub const WD: usize = 0xDC;
    pub const WD_TYPE: &'static str = "3/4";
    pub const WD_STRE: &'static str = "WD";

    pub const RD: usize = 0xD8;
    pub const RD_TYPE: &'static str = "3/4";
    pub const RD_STRE: &'static str = "RD";

    // STORE
    pub const STA: usize = 0x0C; // m..m+2 <-- (A)
    pub const STA_TYPE: &'static str = "3/4";
    pub const STA_STR: &'static str = "STA";

    pub const STB: usize = 0x78; // m..m+2 <-- (B)
    pub const STB_TYPE: &'static str = "3/4";
    pub const STB_STR: &'static str = "STB";

    pub const STX: usize = 0x10; // m..m+2 <-- (X)
    pub const STX_TYPE: &'static str = "3/4";
    pub const STX_STR: &'static str = "STX";

    pub const STL: usize = 0x14; // m..m+2 <-- (L)
    pub const STL_TYPE: &'static str = "3/4";
    pub const STL_STR: &'static str = "STL";

    pub const STCH: usize = 0x54; // m <-- (A) [rightmost byte]
    pub const STCH_TYPE: &'static str = "3/4";
    pub const STCH_STR: &'static str = "STCH";

    pub const STT: usize = 0x84; // m..m+2 <-- (T)
    pub const STT_TYPE: &'static str = "3/4";
    pub const STT_STR: &'static str = "STT";

    pub const STSW: usize = 0xE8; // m..m+2 <-- (SW)
    pub const STSW_TYPE: &'static str = "3/4";
    pub const STSW_STR: &'static str = "STSW";

    pub const STS: usize = 0x7C; // m..m+2 <-- (S)
    pub const STS_TYPE: &'static str = "3/4";
    pub const STS_STR: &'static str = "STS";
}
use std::collections::HashMap;

pub struct RegisterManager {
    pub reg_name: HashMap<usize, String>,
}
// pub const A: usize = 0; // Register A
// pub const X: usize = 1; // Register X
// pub const L: usize = 2; // Register L
// pub const B: usize = 3; // Register B
// pub const S: usize = 4; // Register S
// pub const T: usize = 5; // Register T
// pub const F: usize = 6; // Register F
// pub const PC: usize = 8; // Program Counter
// pub const SW: usize = 9; // Status Word
impl RegisterManager {
    pub fn new() -> Self {
        let mut reg_name = HashMap::new();
        reg_name.insert(0, String::from("A"));
        reg_name.insert(1, String::from("X"));
        reg_name.insert(2, String::from("L"));
        reg_name.insert(3, String::from("B"));
        reg_name.insert(4, String::from("S"));
        reg_name.insert(5, String::from("T"));
        reg_name.insert(6, String::from("F"));
        //reg_name.insert(7, "String::from(PC"));
        reg_name.insert(8, String::from("PC"));
        reg_name.insert(9, String::from("SW"));

        RegisterManager { reg_name: reg_name }
    }
}
