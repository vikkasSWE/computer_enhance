use std::error::Error;

use crate::mov::{ImmediateToRegister, RegisterMemoryToFromRegister};

#[derive(Debug)]
#[allow(unused)]
pub enum Instruction {
    RegisterMemoryToFromRegister(RegisterMemoryToFromRegister),
    // ImmediateToRegisterMemory(ImmediateToRegisterMemory),
    ImmediateToRegister(ImmediateToRegister),
    // MemoryToAccumulator,
    // AccumulatorToMemory,
    // RegisterMemoryToSegmentRegister,
    // SegmentRegisterToRegisterMemory,
}
impl Instruction {
    pub(crate) fn decode(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let res = match bytes[0] >> 0 {
            _ => match bytes[0] >> 1 {
                _ => match bytes[0] >> 2 {
                    0b100010 => Self::RegisterMemoryToFromRegister(
                        RegisterMemoryToFromRegister::decode(bytes),
                    ),
                    _ => match bytes[0] >> 3 {
                        _ => match bytes[0] >> 4 {
                            0b1011 => Self::ImmediateToRegister(ImmediateToRegister::decode(bytes)),

                            _ => {
                                eprintln!("{:0b}", bytes[0]);

                                unimplemented!()
                            }
                        },
                    },
                },
            },
        };

        Ok(res)
    }

    pub(crate) fn offset(&self) -> usize {
        match self {
            Instruction::RegisterMemoryToFromRegister(i) => i.offset(),
            Instruction::ImmediateToRegister(i) => i.offset(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            Instruction::RegisterMemoryToFromRegister(i) => i.to_string(),
            Instruction::ImmediateToRegister(i) => i.to_string(),
        }
    }
}
