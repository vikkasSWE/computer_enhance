use std::error::Error;

use instruction::Instruction;

mod instruction;
mod tests;

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

        offset += 2;

        res.push_str(&instruction.to_string())
    }

    Ok(res)
}
