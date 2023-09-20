#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Mem,
    Reg,
    Mem8,
    Mem16,
}
impl Mode {
    pub fn decode(bits: u8) -> Self {
        match bits {
            0b00 => Self::Mem,
            0b01 => Self::Mem8,
            0b10 => Self::Mem16,
            0b11 => Self::Reg,
            _ => unreachable!(),
        }
    }
}
