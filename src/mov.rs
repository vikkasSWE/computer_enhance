use crate::{mode::Mode, register::Register};

#[derive(Debug)]
pub struct RegisterMemoryToFromRegister {
    d: bool,
    w: bool,
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

        let rm = Register::decode_reg((bytes[1] & 0b0000_0111) >> 0, w);
        let mode = {
            let mut mode = Mode::decode((bytes[1] & 0b1100_0000) >> 6);

            if mode == Mode::Mem {
                if rm == Register::DH || rm == Register::SI {
                    mode = Mode::DirectAddress
                }
            }

            mode
        };
        let reg = Register::decode_reg((bytes[1] & 0b0011_1000) >> 3, w);

        let (disp_lo, disp_hi) = match mode {
            Mode::Reg | Mode::Mem => (None, None),
            Mode::Mem8 => (Some(bytes[2]), None),
            Mode::Mem16 | Mode::DirectAddress => (Some(bytes[2]), Some(bytes[3])),
        };

        Self {
            d,
            w,
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
            Mode::DirectAddress => 2,
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
                        self.rm.memory_mode_to_string(self.mode, None, None, self.w),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm.memory_mode_to_string(self.mode, None, None, self.w),
                    )
                }
            }
            Mode::Mem8 => {
                if self.d {
                    (
                        self.rm
                            .memory_mode_to_string(self.mode, self.disp_lo, None, self.w),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm
                            .memory_mode_to_string(self.mode, self.disp_lo, None, self.w),
                    )
                }
            }
            Mode::Mem16 => {
                if self.d {
                    (
                        self.rm.memory_mode_to_string(
                            self.mode,
                            self.disp_lo,
                            self.disp_hi,
                            self.w,
                        ),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm.memory_mode_to_string(
                            self.mode,
                            self.disp_lo,
                            self.disp_hi,
                            self.w,
                        ),
                    )
                }
            }
            Mode::DirectAddress => {
                if self.d {
                    (
                        self.rm.memory_mode_to_string(
                            self.mode,
                            self.disp_lo,
                            self.disp_hi,
                            self.w,
                        ),
                        self.reg.register_mode_to_string(),
                    )
                } else {
                    (
                        self.reg.register_mode_to_string(),
                        self.rm.memory_mode_to_string(
                            self.mode,
                            self.disp_lo,
                            self.disp_hi,
                            self.w,
                        ),
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
            (bytes[1] as i8) as u16
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

#[derive(Debug)]
pub struct ImmediateToRegisterMemory {
    w: bool,
    mode: Mode,
    rm: Register,
    disp_lo: Option<u8>,
    disp_hi: Option<u8>,
    data: u16,
}

impl ImmediateToRegisterMemory {
    pub(crate) fn decode(bytes: &[u8]) -> Self {
        let w = (bytes[0] & 0b0000_0001) == 0b0000_0001;

        let rm = Register::decode_reg((bytes[1] & 0b0000_0111) >> 0, w);
        let mode = {
            let mut mode = Mode::decode((bytes[1] & 0b1100_0000) >> 6);

            if mode == Mode::Mem {
                if rm == Register::DH || rm == Register::SI {
                    mode = Mode::DirectAddress
                }
            }

            mode
        };

        let (disp_lo, disp_hi, disp_offset) = match mode {
            Mode::Mem | Mode::Reg => (None, None, 0),
            Mode::Mem8 => (Some(bytes[2]), None, 1),
            Mode::Mem16 | Mode::DirectAddress => (Some(bytes[2]), Some(bytes[3]), 2),
        };

        let data = if w {
            u16::from_le_bytes([bytes[2 + disp_offset], bytes[2 + disp_offset + 1]])
        } else {
            let res = bytes[2 + disp_offset] as i8;
            let res = res as i16;
            res as u16
        };

        Self {
            w,
            mode,
            rm,
            disp_lo,
            disp_hi,
            data,
        }
    }

    pub(crate) fn offset(&self) -> usize {
        let mut res = 3;

        if self.w {
            res += 1;
        }

        res += match self.mode {
            Mode::Reg => 0,
            Mode::Mem => 0,
            Mode::Mem8 => 1,
            Mode::Mem16 => 2,
            Mode::DirectAddress => 2,
        };

        res
    }

    pub(crate) fn to_string(&self) -> String {
        let mut res = String::new();

        let dst = self
            .rm
            .memory_mode_to_string(self.mode, self.disp_lo, self.disp_hi, self.w);

        let src = if self.w {
            format!("word + {}", self.data)
        } else {
            format!("byte + {}", self.data)
        };

        res.push_str(&format!("mov {}, {}\r\n", dst, src));
        res
    }
}

#[derive(Debug)]
pub struct MemoryToAccumulator {
    w: bool,
    addr_lo: u8,
    addr_hi: u8,
}

impl MemoryToAccumulator {
    pub(crate) fn decode(bytes: &[u8]) -> Self {
        let w = (bytes[0] & 0b0000_0001) == 0b0000_0001;

        let addr_lo = bytes[1];
        let addr_hi = bytes[2];

        Self {
            w,
            addr_lo,
            addr_hi,
        }
    }

    pub(crate) fn offset(&self) -> usize {
        3
    }
    pub(crate) fn to_string(&self) -> String {
        let mut res = String::new();

        let (src, dst) = (
            u16::from_be_bytes([self.addr_hi, self.addr_lo]),
            if self.w { Register::AX } else { Register::AL }.register_mode_to_string(),
        );

        res.push_str(&format!("mov {}, [{}]\r\n", dst, src));
        res
    }
}
#[derive(Debug)]
pub struct AccumulatorToMemory {
    w: bool,
    addr_lo: u8,
    addr_hi: u8,
}

impl AccumulatorToMemory {
    pub(crate) fn decode(bytes: &[u8]) -> Self {
        let w = (bytes[0] & 0b0000_0001) == 0b0000_0001;

        let addr_lo = bytes[1];
        let addr_hi = bytes[2];

        Self {
            w,
            addr_lo,
            addr_hi,
        }
    }

    pub(crate) fn offset(&self) -> usize {
        3
    }
    pub(crate) fn to_string(&self) -> String {
        let mut res = String::new();

        let (src, dst) = (
            if self.w { Register::AX } else { Register::AL }.register_mode_to_string(),
            u16::from_be_bytes([self.addr_hi, self.addr_lo]),
        );

        res.push_str(&format!("mov [{}], {}\r\n", dst, src));
        res
    }
}
#[derive(Debug)]
pub struct RegisterMemoryToSegmentRegister {}

impl RegisterMemoryToSegmentRegister {
    pub(crate) fn decode(_bytes: &[u8]) -> Self {
        todo!()
    }

    pub(crate) fn offset(&self) -> usize {
        todo!()
    }
    pub(crate) fn to_string(&self) -> String {
        todo!()
    }
}
#[derive(Debug)]
pub struct SegmentRegisterToRegisterMemory {}

impl SegmentRegisterToRegisterMemory {
    pub(crate) fn decode(_bytes: &[u8]) -> Self {
        todo!()
    }

    pub(crate) fn offset(&self) -> usize {
        todo!()
    }
    pub(crate) fn to_string(&self) -> String {
        todo!()
    }
}
