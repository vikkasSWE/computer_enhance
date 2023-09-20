use crate::{mode::Mode, register::Register};

#[derive(Debug)]
pub struct RegisterMemoryToFromRegister {
    d: bool,
    _w: bool,
    mode: Mode,
    reg: Register,
    rm: Register,
    disp_lo: Option<u8>,
    disp_hi: Option<u8>,
}

impl RegisterMemoryToFromRegister {
    pub fn decode(bytes: &[u8]) -> Self {
        let d = (bytes[0] & 0b0000_0010) == 0b0000_0010;
        let w = (bytes[0] & 0b0000_0001) == 0b0000_0001;

        let mode = Mode::decode((bytes[1] & 0b1100_0000) >> 6);
        let reg = Register::decode_reg((bytes[1] & 0b0011_1000) >> 3, w);
        let rm = Register::decode_reg((bytes[1] & 0b0000_0111) >> 0, w);

        let (disp_lo, disp_hi) = match mode {
            Mode::Mem | Mode::Reg => (None, None),
            Mode::Mem8 => (Some(bytes[2]), None),
            Mode::Mem16 => (Some(bytes[2]), Some(bytes[3])),
        };

        Self {
            d,
            _w: w,
            mode,
            reg,
            rm,
            disp_lo,
            disp_hi,
        }
    }

    pub(crate) fn offset(&self) -> usize {
        let mut res = 2;

        res += match self.mode {
            Mode::Reg => 0,
            Mode::Mem => 0,
            Mode::Mem8 => 1,
            Mode::Mem16 => 2,
        };

        res
    }

    pub(crate) fn to_string(&self) -> String {
        let mut res = String::new();

        let (src, dst) = match self.mode {
            Mode::Reg => {
                if self.d {
                    (
                        self.rm.register_mode_to_string(),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm.register_mode_to_string(),
                    )
                }
            }
            Mode::Mem => {
                if self.d {
                    (
                        self.rm.memory_mode_to_string(self.mode, None, None),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm.memory_mode_to_string(self.mode, None, None),
                    )
                }
            }
            Mode::Mem8 => {
                if self.d {
                    (
                        self.rm.memory_mode_to_string(self.mode, self.disp_lo, None),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm.memory_mode_to_string(self.mode, self.disp_lo, None),
                    )
                }
            }
            Mode::Mem16 => {
                if self.d {
                    (
                        self.rm
                            .memory_mode_to_string(self.mode, self.disp_lo, self.disp_hi),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm
                            .memory_mode_to_string(self.mode, self.disp_lo, self.disp_hi),
                    )
                }
            }
        };

        res.push_str(&format!("mov {}, {}\r\n", dst, src));
        res
    }
}

#[derive(Debug)]
pub struct ImmediateToRegister {
    w: bool,
    reg: Register,
    data: u16,
}

impl ImmediateToRegister {
    pub(crate) fn decode(bytes: &[u8]) -> Self {
        let w = bytes[0] & 0b0000_1000 == 0b0000_1000;
        let reg = Register::decode_reg((bytes[0] & 0b0000_0111) >> 0, w);

        let data = if w {
            u16::from_le_bytes([bytes[1], bytes[2]])
        } else {
            u16::from_le_bytes([bytes[1], 0])
        };

        Self { w, reg, data }
    }

    pub(crate) fn offset(&self) -> usize {
        let mut res = 2;

        if self.w {
            res += 1;
        }

        res
    }

    pub(crate) fn to_string(&self) -> String {
        let mut res = String::new();

        let dst = self.reg.register_mode_to_string();
        let src = self.data.to_string();

        res.push_str(&format!("mov {}, {}\r\n", dst, src));
        res
    }
}
