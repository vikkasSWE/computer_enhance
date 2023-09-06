use std::fmt::{Binary, Display};

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Short(u8),
    Wide(u16),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Short(s) => write!(f, "{}", s),
            Value::Wide(w) => write!(f, "{}", w),
        }
    }
}

impl Binary for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Short(s) => write!(f, "{s:b}"),
            Value::Wide(w) => write!(f, "{w:b}"),
        }
    }
}

impl Value {
    pub fn from(w: bool, bytes: &[u8]) -> Self {
        if w {
            Self::Wide(u16::from_le_bytes([bytes[0], bytes[1]]))
        } else {
            Self::Short(bytes[0])
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    Value(Value),
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AL => write!(f, "al"),
            Register::CL => write!(f, "cl"),
            Register::DL => write!(f, "dl"),
            Register::BL => write!(f, "bl"),
            Register::AH => write!(f, "ah"),
            Register::CH => write!(f, "ch"),
            Register::DH => write!(f, "dh"),
            Register::BH => write!(f, "bh"),

            Register::AX => write!(f, "ax"),
            Register::CX => write!(f, "cx"),
            Register::DX => write!(f, "dx"),
            Register::BX => write!(f, "bx"),
            Register::SP => write!(f, "sp"),
            Register::BP => write!(f, "bp"),
            Register::SI => write!(f, "si"),
            Register::DI => write!(f, "di"),
            Register::Value(v) => write!(f, "{v}"),
        }
    }
}

impl Binary for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AL => write!(f, "000"),
            Register::CL => write!(f, "001"),
            Register::DL => write!(f, "010"),
            Register::BL => write!(f, "011"),
            Register::AH => write!(f, "100"),
            Register::CH => write!(f, "101"),
            Register::DH => write!(f, "110"),
            Register::BH => write!(f, "111"),

            Register::AX => write!(f, "000"),
            Register::CX => write!(f, "001"),
            Register::DX => write!(f, "010"),
            Register::BX => write!(f, "011"),
            Register::SP => write!(f, "100"),
            Register::BP => write!(f, "101"),
            Register::SI => write!(f, "110"),
            Register::DI => write!(f, "111"),
            Register::Value(v) => write!(f, "{v:b}"),
        }
    }
}

impl Register {
    pub fn new(byte: u8, w: bool) -> Self {
        if w {
            match byte {
                0b000 => Self::AX,
                0b001 => Self::CX,
                0b010 => Self::DX,
                0b011 => Self::BX,
                0b100 => Self::SP,
                0b101 => Self::BP,
                0b110 => Self::SI,
                0b111 => Self::DI,

                _ => panic!(),
            }
        } else {
            match byte {
                0b000 => Self::AL,
                0b001 => Self::CL,
                0b010 => Self::DL,
                0b011 => Self::BL,
                0b100 => Self::AH,
                0b101 => Self::CH,
                0b110 => Self::DH,
                0b111 => Self::BH,

                _ => panic!(),
            }
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Register::AL => "al".to_string(),
            Register::CL => "cl".to_string(),
            Register::DL => "dl".to_string(),
            Register::BL => "bl".to_string(),

            Register::AH => "ah".to_string(),
            Register::CH => "ch".to_string(),
            Register::DH => "dh".to_string(),
            Register::BH => "bh".to_string(),

            Register::AX => "ax".to_string(),
            Register::CX => "cx".to_string(),
            Register::DX => "dx".to_string(),
            Register::BX => "bx".to_string(),

            Register::SP => "sp".to_string(),
            Register::BP => "bp".to_string(),
            Register::SI => "si".to_string(),
            Register::DI => "di".to_string(),
            Register::Value(v) => format!("{}", v),
        }
    }
}
