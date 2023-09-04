use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum OpCode {
    MOV,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            OpCode::MOV => "100010",
        };

        write!(f, "{res}")
    }
}

impl OpCode {
    fn read_op_code(byte: u8) -> OpCode {
        match byte >> 2 {
            0b00100010 => OpCode::MOV,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct OneByteInstruction {
    pub opcode: OpCode,
    pub d: u8,
    pub w: u8,
}

#[derive(Debug)]
pub struct TwoByteInstruction {
    pub opcode: OpCode,
    pub d: u8,
    pub w: u8,
    pub mode: u8,
    pub reg: u8,
    pub rm: u8,
}

impl Display for TwoByteInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "opcode d w mod reg r/m\n{} {:0b} {:0b}  {:02b} {:03b} {:03b}",
            self.opcode, self.d, self.w, self.mode, self.reg, self.rm
        )
    }
}

impl TwoByteInstruction {
    fn to_string(&self) -> String {
        let operation = match self.opcode {
            OpCode::MOV => "mov",
        };

        let dst = {
            let mut res = String::new();

            res.push_str(if self.w == 0 {
                match self.rm {
                    0b000 => "al",
                    0b001 => "cl",
                    0b010 => "dl",
                    0b011 => "bl",
                    0b100 => "ah",
                    0b101 => "ch",
                    0b110 => "dh",
                    0b111 => "bh",

                    _ => panic!(),
                }
            } else {
                match self.rm {
                    0b000 => "ax",
                    0b001 => "cx",
                    0b010 => "dx",
                    0b011 => "bx",
                    0b100 => "sp",
                    0b101 => "bp",
                    0b110 => "si",
                    0b111 => "di",

                    _ => panic!(),
                }
            });

            res
        };

        let src = {
            let mut res = String::new();

            res.push_str(if self.w == 0 {
                match self.reg {
                    0b000 => "al",
                    0b001 => "cl",
                    0b010 => "dl",
                    0b011 => "bl",
                    0b100 => "ah",
                    0b101 => "ch",
                    0b110 => "dh",
                    0b111 => "bh",

                    _ => panic!(),
                }
            } else {
                match self.reg {
                    0b000 => "ax",
                    0b001 => "cx",
                    0b010 => "dx",
                    0b011 => "bx",
                    0b100 => "sp",
                    0b101 => "bp",
                    0b110 => "si",
                    0b111 => "di",

                    _ => panic!(),
                }
            });

            res
        };

        format!("{operation} {dst}, {src}\r\n")
    }
}

#[derive(Debug)]
pub enum Instruction {
    OneByteInstruction(OneByteInstruction),
    TwoByteInstruction(TwoByteInstruction),
}

impl Instruction {
    pub fn decode(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let opcode = OpCode::read_op_code(bytes[0]);

        let d = (bytes[0] & 0b00000010) >> 1;
        let w = (bytes[0] & 0b00000001) >> 0;

        // if w == 0 {
        //     return Ok(Self::OneByteInstruction(OneByteInstruction {
        //         opcode,
        //         d,
        //         w,
        //     }));
        // }

        let mode = (bytes[1] & 0b11000000) >> 6;
        let reg = (bytes[1] & 0b00111000) >> 3;
        let rm = (bytes[1] & 0b00000111) >> 0;

        Ok(Self::TwoByteInstruction(TwoByteInstruction {
            opcode,
            d,
            w,
            mode,
            reg,
            rm,
        }))
    }

    pub fn to_string(&self) -> String {
        match self {
            Instruction::OneByteInstruction(_) => unimplemented!(),
            Instruction::TwoByteInstruction(two) => two.to_string(),
        }
    }
}

pub fn dissassemble(bytes: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();
    res.push_str("bits 16\r\n\r\n");

    let mut offset = 0;
    loop {
        if offset >= bytes.len() {
            break;
        }

        let instruction_bytes = bytes[offset..].to_vec();
        let Ok(instruction) = Instruction::decode(&instruction_bytes) else {break};

        offset += match instruction {
            Instruction::OneByteInstruction(_) => 1,
            Instruction::TwoByteInstruction(_) => 2,
        };

        res.push_str(&instruction.to_string())
    }

    Ok(res)
}
