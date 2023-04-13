use crate::location::Location;

pub struct Instruction {
    location_1: Location,
    location_2: Location,
    direction: bool,
    word: Option<bool>,
}

pub const MOV_RM_TO_RM: u8 =   0b10001000;
pub const MOV_IMM_TO_REG: u8 = 0b10110000;
pub const MOV_IMM_TO_RM: u8 =  0b11000110;
pub const MOV_MEM_TO_ACC: u8 = 0b10100000;
pub const MOV_ACC_TO_MEM: u8 = 0b10100010;
