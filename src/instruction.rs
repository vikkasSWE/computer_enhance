use std::error::Error;

use crate::mov::{
    AccumulatorToMemory, ImmediateToRegister, ImmediateToRegisterMemory, MemoryToAccumulator,
    RegisterMemoryToFromRegister, RegisterMemoryToSegmentRegister, SegmentRegisterToRegisterMemory,
};

#[derive(Debug)]
#[allow(unused)]
pub enum Instruction {
    RegisterMemoryToFromRegister(RegisterMemoryToFromRegister),
    ImmediateToRegisterMemory(ImmediateToRegisterMemory),
    ImmediateToRegister(ImmediateToRegister),
    MemoryToAccumulator(MemoryToAccumulator),
    AccumulatorToMemory(AccumulatorToMemory),
    RegisterMemoryToSegmentRegister(RegisterMemoryToSegmentRegister),
    SegmentRegisterToRegisterMemory(SegmentRegisterToRegisterMemory),
}
impl Instruction {
    pub(crate) fn decode(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let res = match bytes[0] >> 0 {
            0b10001110 => Self::RegisterMemoryToSegmentRegister(
                RegisterMemoryToSegmentRegister::decode(bytes),
            ),
            0b10001100 => Self::SegmentRegisterToRegisterMemory(
                SegmentRegisterToRegisterMemory::decode(bytes),
            ),
            _ => match bytes[0] >> 1 {
                0b1100011 => {
                    Self::ImmediateToRegisterMemory(ImmediateToRegisterMemory::decode(bytes))
                }
                0b1010000 => Self::MemoryToAccumulator(MemoryToAccumulator::decode(bytes)),
                0b1010001 => Self::AccumulatorToMemory(AccumulatorToMemory::decode(bytes)),

                _ => match bytes[0] >> 2 {
                    0b100010 => Self::RegisterMemoryToFromRegister(
                        RegisterMemoryToFromRegister::decode(bytes),
                    ),
                    _ => match bytes[0] >> 3 {
                        _ => match bytes[0] >> 4 {
                            0b1011 => Self::ImmediateToRegister(ImmediateToRegister::decode(bytes)),

                            _ => {
                                eprintln!("Instruction Not Implemented: {:08b}", bytes[0]);

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
            Instruction::ImmediateToRegisterMemory(i) => i.offset(),
            Instruction::MemoryToAccumulator(i) => i.offset(),
            Instruction::AccumulatorToMemory(i) => i.offset(),
            Instruction::RegisterMemoryToSegmentRegister(i) => i.offset(),
            Instruction::SegmentRegisterToRegisterMemory(i) => i.offset(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            Instruction::RegisterMemoryToFromRegister(i) => i.to_string(),
            Instruction::ImmediateToRegister(i) => i.to_string(),
            Instruction::ImmediateToRegisterMemory(i) => i.to_string(),
            Instruction::MemoryToAccumulator(i) => i.to_string(),
            Instruction::AccumulatorToMemory(i) => i.to_string(),
            Instruction::RegisterMemoryToSegmentRegister(i) => i.to_string(),
            Instruction::SegmentRegisterToRegisterMemory(i) => i.to_string(),
        }
    }
}
