use crate::register::{Register, MemoryOffsetRegister};

pub enum ByteOrWord {
    Byte(u8),
    Word(u16),
    ByteOffset(i8),
    WordOffset(i16),
}

impl ByteOrWord {
    pub fn is_zero(&self) -> bool {
        match *self {
            ByteOrWord::Byte(v) => v == 0,
            ByteOrWord::Word(v) => v == 0,
            ByteOrWord::ByteOffset(v) => v == 0,
            ByteOrWord::WordOffset(v) => v == 0,
        }
    }
    pub fn is_negative(&self) -> bool {
        match *self {
            ByteOrWord::ByteOffset(v) => v < 0,
            ByteOrWord::WordOffset(v) => v < 0,
            _ => false
        }
    }
}

impl std::fmt::Display for ByteOrWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match *self {
            ByteOrWord::Byte(v) => v as u16,
            ByteOrWord::Word(v) => v,
            ByteOrWord::ByteOffset(v) => v.abs() as u16,
            ByteOrWord::WordOffset(v) => v.abs() as u16,
        };
        
        write!(f, "{val}")
    }
}

pub enum Location {
    Register(Register),
    Memory(Option<MemoryOffsetRegister>, Option<ByteOrWord>),
    Accumulator(bool),
    Immediate(ByteOrWord),
} 

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Location::Register(reg) => format!("{reg}"),
            Location::Memory(None, Some(memory_address)) => format!("[{memory_address}]"),
            Location::Memory(Some(reg), None) => format!("[{reg}]"),
            Location::Memory(Some(reg), Some(offset)) => match offset.is_negative() {
                true => format!("[{reg} - {offset}]"),
                false => format!("[{reg} + {offset}]")
            }
            Location::Accumulator(word) => if *word { "ax".to_string()} else { "al".to_string()},
            Location::Immediate(value) => format!("{value}"),
            _ => unreachable!(),
        };

        write!(f, "{out}")
    }
}