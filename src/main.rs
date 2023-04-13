use crate::{register::{Register, MemoryOffsetRegister}, location::{Location, ByteOrWord}};

mod bitmask;
mod instruction;
mod location;
mod register;
mod utils;

fn main() {
    let data = match utils::read_bytes_from_env_file() {
        Ok(d) => d,
        Err(msg) => panic!("{msg}")
    };

    let mut program_counter = data.iter();

    println!("bits 16\n");

    while let Some(byte) = program_counter.next() {
        match byte & bitmask::LAST_FOUR_ZERO {
            instruction::MOV_IMM_TO_REG => {
                let word = byte & 8 == 8;
                let reg = byte & bitmask::LAST_THREE_ONE;
                let reg = Register::new(reg, word);

                let mut data = *program_counter.next().unwrap() as u16;
                if word {
                    data = ((*program_counter.next().unwrap() as u16) << 8) | data;
                }

                println!("mov {reg}, {data}");
                continue;
            }
            _ => {}
        }
        match byte & bitmask::LAST_TWO_ZERO {
            instruction::MOV_RM_TO_RM => {
                let direction = byte & 2 == 2;
                let word = byte & 1 == 1;
                let data = program_counter.next().unwrap();
                let mode = data >> 6;
                let reg = (data & bitmask::FIRST_TWO_ZERO) >> 3;
                let reg = Register::new(reg, word);
                let rm = data & bitmask::LAST_THREE_ONE;


                let displacement = match (mode, rm) {
                    (3, _) => None,
                    (0, 6) => {
                        let data = *program_counter.next().unwrap() as u16;
                        let displacement = ((*program_counter.next().unwrap() as u16) << 8) | data;
                        Some(ByteOrWord::Word(displacement))
                    }
                    (0, _) => None,
                    (1, _) => {
                        Some(ByteOrWord::ByteOffset(*program_counter.next().unwrap() as i8))
                    },
                    (2, _) => {
                        let data = *program_counter.next().unwrap() as u16;
                        let displacement = ((*program_counter.next().unwrap() as u16) << 8) | data;
                        Some(ByteOrWord::WordOffset(displacement as i16))
                    }
                    _ => unreachable!()
                };

                let location = match mode {
                    3 => Location::Register(Register::new(rm, word)),
                    _ => Location::Memory(MemoryOffsetRegister::new(rm, mode), displacement),
                };

                if direction {
                    println!("mov {reg}, {location}")
                } else {
                    println!("mov {location}, {reg}")
                }
                continue;
            }
            _ => {}
        }
        match byte & bitmask::LAST_ONE_ZERO {
            instruction::MOV_IMM_TO_RM => {
                let word = byte & 1 == 1;
                let data = program_counter.next().unwrap();
                let mode = data >> 6;
                let reg = (data & bitmask::FIRST_TWO_ZERO) >> 3;
                assert_eq!(reg, 0);
                let rm = data & bitmask::LAST_THREE_ONE;


                let displacement = match (mode, rm) {
                    (3, _) => None,
                    (0, 6) => {
                        let data = *program_counter.next().unwrap() as u16;
                        let displacement = ((*program_counter.next().unwrap() as u16) << 8) | data;
                        Some(ByteOrWord::Word(displacement))
                    }
                    (0, _) => None,
                    (1, _) => {
                        Some(ByteOrWord::ByteOffset(*program_counter.next().unwrap() as i8))
                    },
                    (2, _) => {
                        let data = *program_counter.next().unwrap() as u16;
                        let displacement = ((*program_counter.next().unwrap() as u16) << 8) | data;
                        Some(ByteOrWord::WordOffset(displacement as i16))
                    }
                    _ => unreachable!()
                };

                let mut data = *program_counter.next().unwrap() as u16;
                if word {
                    data = ((*program_counter.next().unwrap() as u16) << 8) | data;
                }

                let size = if word { "word" } else {"byte"};

                let location = match mode {
                    3 => Location::Register(Register::new(rm, word)),
                    _ => Location::Memory(MemoryOffsetRegister::new(rm, mode), displacement),
                };

                println!("mov {location}, {size} {data}");
                continue;
            }
            instruction::MOV_MEM_TO_ACC => {
                let word = byte & 1 == 1;
                let mut address = *program_counter.next().unwrap() as u16;
                address = ((*program_counter.next().unwrap() as u16) << 8) | address;
                let acc = Location::Accumulator(word);
                println!("mov {acc}, [{address}]");
                continue;
            }
            instruction::MOV_ACC_TO_MEM => {
                let word = byte & 1 == 1;
                let mut address = *program_counter.next().unwrap() as u16;
                address = ((*program_counter.next().unwrap() as u16) << 8) | address;
                let acc = Location::Accumulator(word);
                println!("mov [{address}], {acc}");
                continue;
            }
            _ => {}
        }
        match byte {
            _ => {}
        }
    }
}
