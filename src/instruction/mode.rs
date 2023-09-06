use std::fmt::{Binary, Display};

use super::register::Register;

#[derive(Debug)]
pub enum Mode {
    Memory,
    Memory8Bit,
    Memory16Bit,
    Register,
}
impl Mode {
    pub(crate) fn get_effective_address(rm: &Register) -> String {
        match rm {
            Register::AL => "[bx + si]",
            Register::CL => "[bx + di]",
            Register::DL => "[bp + si]",
            Register::BL => "[bp + di]",

            Register::AH => "[si]",
            Register::CH => "[di]",
            Register::DH => "[bp]",
            Register::BH => "[bx]",

            Register::AX => "[bx + si]",
            Register::CX => "[bx + di]",
            Register::DX => "[bp + si]",
            Register::BX => "[bp + di]",

            Register::SP => "[si]",
            Register::BP => "[di]",
            Register::SI => "[bp]",
            Register::DI => "[bx]",
            Register::Value(_) => unreachable!(),
        }
        .to_string()
    }
}

impl Mode {
    pub fn from(value: u8) -> Self {
        match value {
            0b00 => Self::Memory,
            0b01 => Self::Memory8Bit,
            0b10 => Self::Memory16Bit,
            0b11 => Self::Register,
            _ => unreachable!(),
        }
    }
}

impl Binary for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Memory => write!(f, "00"),
            Mode::Memory8Bit => write!(f, "01"),
            Mode::Memory16Bit => write!(f, "10"),
            Mode::Register => write!(f, "11"),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Memory => write!(f, "00"),
            Mode::Memory8Bit => write!(f, "01"),
            Mode::Memory16Bit => write!(f, "10"),
            Mode::Register => write!(f, "11"),
        }
    }
}
