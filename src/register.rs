pub enum Register {
    AL,
    AH,
    AX,
    BL,
    BH,
    BX,
    CL,
    CH,
    CX,
    DL,
    DH,
    DX,
    SP,
    BP,
    SI,
    DI
}

impl Register {
    pub fn new(reg: u8, word: bool) -> Register {
        match (reg, word) {
            // 8-bit
            (0, false) => Register::AL,
            (1, false) => Register::CL,
            (2, false) => Register::DL,
            (3, false) => Register::BL,
            (4, false) => Register::AH,
            (5, false) => Register::CH,
            (6, false) => Register::DH,
            (7, false) => Register::BH,
            // 16-bit
            (0, true) => Register::AX,
            (1, true) => Register::CX,
            (2, true) => Register::DX,
            (3, true) => Register::BX,
            (4, true) => Register::SP,
            (5, true) => Register::BP,
            (6, true) => Register::SI,
            (7, true) => Register::DI,
            _ => unreachable!()
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = match self {
            Register::AL => "al",
            Register::AH => "ah",
            Register::AX => "ax",
            Register::BL => "bl",
            Register::BH => "bh",
            Register::BX => "bx",
            Register::CL => "cl",
            Register::CH => "ch",
            Register::CX => "cx",
            Register::DL => "dl",
            Register::DH => "dh",
            Register::DX => "dx",
            Register::SP => "sp",
            Register::BP => "bp",
            Register::SI => "si",
            Register::DI => "di",
        };

        write!(f, "{reg}")
    }
}

pub enum MemoryOffsetRegister {
    BXSI,
    BXDI,
    BPSI,
    BPDI,
    SI,
    DI,
    BP,
    BX
}

impl MemoryOffsetRegister {
    pub fn new(rm: u8, mode: u8) -> Option<MemoryOffsetRegister> {
        match (rm, mode) {
            (6, 0) => None,
            (0, _) => Some(MemoryOffsetRegister::BXSI),
            (1, _) => Some(MemoryOffsetRegister::BXDI),
            (2, _) => Some(MemoryOffsetRegister::BPSI),
            (3, _) => Some(MemoryOffsetRegister::BPDI),
            (4, _) => Some(MemoryOffsetRegister::SI),
            (5, _) => Some(MemoryOffsetRegister::DI),
            (6, _) => Some(MemoryOffsetRegister::BP),
            (7, _) => Some(MemoryOffsetRegister::BX),
            _ => unreachable!()
        }
    }
}

impl std::fmt::Display for MemoryOffsetRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = match self {
            MemoryOffsetRegister::BXSI => "bx + si",
            MemoryOffsetRegister::BXDI => "bx + di",
            MemoryOffsetRegister::BPSI => "bp + si",
            MemoryOffsetRegister::BPDI => "bp + di",
            MemoryOffsetRegister::SI => "si",
            MemoryOffsetRegister::DI => "di",
            MemoryOffsetRegister::BP => "bp",
            MemoryOffsetRegister::BX => "bx",
        };

        write!(f, "{reg}")
    }
}