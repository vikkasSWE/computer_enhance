use crate::mode::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl Register {
    pub fn decode_reg(bits: u8, w: bool) -> Self {
        match bits {
            0b000 => {
                if w {
                    Self::AX
                } else {
                    Self::AL
                }
            }
            0b001 => {
                if w {
                    Self::CX
                } else {
                    Self::CL
                }
            }
            0b010 => {
                if w {
                    Self::DX
                } else {
                    Self::DL
                }
            }
            0b011 => {
                if w {
                    Self::BX
                } else {
                    Self::BL
                }
            }
            0b100 => {
                if w {
                    Self::SP
                } else {
                    Self::AH
                }
            }
            0b101 => {
                if w {
                    Self::BP
                } else {
                    Self::CH
                }
            }
            0b110 => {
                if w {
                    Self::SI
                } else {
                    Self::DH
                }
            }
            0b111 => {
                if w {
                    Self::DI
                } else {
                    Self::BH
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn register_mode_to_string(&self) -> String {
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
        }
    }

    pub(crate) fn memory_mode_to_string(
        &self,
        mode: Mode,
        disp_lo: Option<u8>,
        disp_hi: Option<u8>,
        w: bool,
    ) -> String {
        match mode {
            Mode::Mem => match self {
                Register::AL | Register::AX => "[bx + si]".to_string(),
                Register::CL | Register::CX => "[bx + di]".to_string(),
                Register::DL | Register::DX => "[bp + si]".to_string(),
                Register::BL | Register::BX => "[bp + di]".to_string(),
                Register::AH | Register::SP => "[si]".to_string(),
                Register::CH | Register::BP => "[di]".to_string(),
                Register::DH | Register::SI => "[bp]".to_string(),
                Register::BH | Register::DI => "[bx]".to_string(),
            },
            Mode::Mem8 => {
                let d8 = if w {
                    let d8 = disp_lo.unwrap() as i8;
                    let d8 = d8 as i16;
                    let d8 = d8 as i8;
                    d8 as u16
                } else {
                    disp_lo.unwrap() as u16
                };

                match self {
                    Register::AL | Register::AX => format!("[bx + si + {d8}]"),
                    Register::CL | Register::CX => format!("[bx + di + {d8}]"),
                    Register::DL | Register::DX => format!("[bp + si + {d8}]"),
                    Register::BL | Register::BX => format!("[bp + di + {d8}]"),
                    Register::AH | Register::SP => format!("[si + {d8}]"),
                    Register::CH | Register::BP => format!("[di + {d8}]"),
                    Register::DH | Register::SI => format!("[bp + {d8}]"),
                    Register::BH | Register::DI => format!("[bx + {d8}]"),
                }
            }
            Mode::Mem16 => {
                let d16 = u16::from_le_bytes([disp_lo.unwrap(), disp_hi.unwrap()]);
                match self {
                    Register::AL | Register::AX => format!("[bx + si + {d16}]"),
                    Register::CL | Register::CX => format!("[bx + di + {d16}]"),
                    Register::DL | Register::DX => format!("[bp + si + {d16}]"),
                    Register::BL | Register::BX => format!("[bp + di + {d16}]"),
                    Register::AH | Register::SP => format!("[si + {d16}]"),
                    Register::CH | Register::BP => format!("[di + {d16}]"),
                    Register::DH | Register::SI => format!("[bp + {d16}]"),
                    Register::BH | Register::DI => format!("[bx + {d16}]"),
                }
            }

            Mode::Reg => unreachable!(),
            Mode::DirectAddress => format!(
                "[{}]",
                u16::from_le_bytes([disp_lo.unwrap(), disp_hi.unwrap()])
            ),
        }
    }
}
