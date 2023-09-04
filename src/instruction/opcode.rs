use std::fmt::Display;

#[derive(Debug)]
pub enum MovType {
    RegisterMemoryToFromRegister,
    ImmediateToRegister,
}
#[derive(Debug)]
pub enum OpCode {
    MOV(MovType),
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            OpCode::MOV(MovType::RegisterMemoryToFromRegister) => "100010",
            OpCode::MOV(MovType::ImmediateToRegister) => "100010",
        };

        write!(f, "{res}")
    }
}

impl OpCode {
    pub fn read_op_code(byte: u8) -> OpCode {
        // println!("Failed to parse OpCode: {byte:b}");

        match byte >> 2 {
            0b100010 => OpCode::MOV(MovType::RegisterMemoryToFromRegister),

            _ => match byte >> 4 {
                0b1011 => OpCode::MOV(MovType::ImmediateToRegister),
                _ => {
                    println!("Failed to parse OpCode: {:b}", byte >> 2);
                    panic!();
                }
            },
        }
    }
}
