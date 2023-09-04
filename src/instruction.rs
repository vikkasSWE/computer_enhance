use std::{error::Error, fmt::Display};

use crate::instruction::register::Value;

use self::{
    opcode::{MovType, OpCode},
    register::Register,
};

mod opcode;
mod register;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub d: u8,
    pub w: bool,
    pub mode: u8,
    pub reg: Register,
    pub rm: Register,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "opcode d w mod reg r/m\n{} {:0b} {:0b}  {:02b} {:03b} {:03b}",
            self.opcode, self.d, self.w as u8, self.mode, self.reg, self.rm
        )
    }
}

impl Instruction {
    pub fn to_string(&self) -> String {
        let operation = match self.opcode {
            OpCode::MOV(_) => "mov",
        };

        let dst = self.rm.to_str();
        let src = self.reg.to_str();

        format!("{operation} {dst}, {src}\r\n")
    }

    pub fn decode(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let opcode = OpCode::read_op_code(bytes[0]);

        match opcode {
            OpCode::MOV(MovType::RegisterMemoryToFromRegister) => {
                let d = (bytes[0] & 0b00000010) >> 1;
                let w = ((bytes[0] & 0b00000001) >> 0) == 0b00000001;

                let mode = (bytes[1] & 0b11000000) >> 6;
                let reg = Register::new((bytes[1] & 0b00111000) >> 3, w);
                let rm = Register::new((bytes[1] & 0b00000111) >> 0, w);

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
                let d = 0;
                let w = (bytes[0] & 0b00001000) == 0b00001000;

                let mode = 0;
                let reg = Register::Value(Value::from(w, vec![bytes[1]]));
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
}
