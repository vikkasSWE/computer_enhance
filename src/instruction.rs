use std::{error::Error, fmt::Display};

use crate::instruction::register::Value;

use self::{
    mode::Mode,
    opcode::{MovType, OpCode},
    register::Register,
};

mod mode;
mod opcode;
mod register;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub d: bool,
    pub w: bool,
    pub mode: Mode,
    pub reg: Register,
    pub rm: Register,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "opcode d w mod reg r/m\n{} {:0b} {:0b}  {:02b} {:03b} {:03b}",
            self.opcode, self.d as u8, self.w as u8, self.mode, self.reg, self.rm
        )
    }
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match &self.opcode {
            OpCode::MOV(t) => match t {
                MovType::RegisterMemoryToFromRegister => {
                    let (src_reg, dst_reg) = if self.d {
                        (&self.reg, &self.rm)
                    } else {
                        (&self.rm, &self.reg)
                    };

                    let dst = dst_reg.to_str();
                    let src = {
                        match self.mode {
                            Mode::Memory => Mode::get_effective_address(&src_reg),
                            Mode::Memory8Bit => Mode::get_effective_address(&src_reg),
                            Mode::Memory16Bit => unimplemented!(),
                            Mode::Register => src_reg.to_str(),
                        }
                    };

                    format!("mov {dst}, {src}\r\n")
                }
                MovType::ImmediateToRegister => {
                    let dst = self.rm.to_str();
                    let src = self.reg.to_str();
                    format!("mov {dst}, {src}\r\n")
                }
            },
        }
    }

    pub fn decode(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let opcode = OpCode::read_op_code(bytes[0]);

        match opcode {
            OpCode::MOV(MovType::RegisterMemoryToFromRegister) => {
                let d = ((bytes[0] & 0b00000010) >> 1) == 0b0000000;
                let w = ((bytes[0] & 0b00000001) >> 0) == 0b00000001;

                let reg = Register::new((bytes[1] & 0b00111000) >> 3, w);
                let rm_raw = (bytes[1] & 0b00000111) >> 0;
                let rm = Register::new(rm_raw, w);

                let mod_raw = (bytes[1] & 0b11000000) >> 6;

                // println!("Mode: {mod_raw:02b}, R/M: {rm_raw:03b}");
                let mode = Mode::from(mod_raw);

                Ok(Self {
                    opcode,
                    d,
                    w,
                    mode,
                    reg,
                    rm,
                })
            }
            OpCode::MOV(MovType::ImmediateToRegister) => {
                let d = false;
                let w = (bytes[0] & 0b00001000) == 0b00001000;

                let mode = Mode::Memory; // !Note: not actually Memory, non existing
                let reg = Register::Value(if w {
                    Value::from(w, &[bytes[1], bytes[2]])
                } else {
                    Value::from(w, &[bytes[1]])
                });
                let rm = Register::new(bytes[0] & 0b00000111, w);

                Ok(Self {
                    opcode,
                    d,
                    w,
                    mode,
                    reg,
                    rm,
                })
            }
        }
    }

    pub(crate) fn get_offset(&self) -> usize {
        let mut res = 0;
        match &self.opcode {
            OpCode::MOV(t) => match t {
                MovType::RegisterMemoryToFromRegister => {
                    res += 2;

                    res += match self.mode {
                        Mode::Memory => 0,
                        Mode::Memory8Bit => 1,
                        Mode::Memory16Bit => 2,
                        Mode::Register => 0,
                    }
                }
                MovType::ImmediateToRegister => {
                    res += 1;
                    if self.w {
                        res += 2;
                    } else {
                        res += 1;
                    }
                }
            },
        }
        res
    }
}
