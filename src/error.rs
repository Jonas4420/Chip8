use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPadSize(usize, usize),
    InvalidScreenSize((usize, usize), (usize, usize)),
    PadOutOfRange(u8),
    RamOutOfRange(u16),
    StackOverflow,
    UndefinedInstruction([u8; 4]),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPadSize(size, supported) => {
                write!(f, "Pad size is {}, only size {} is supported", size, supported)
            }
            Self::InvalidScreenSize(size, supported) => {
                write!(f, "Screen size is {:?}, only size {:?} is supported", size, supported)
            }
            Self::PadOutOfRange(addr) => {
                write!(f, "RAM address 0x{:04x} is invalid", addr)
            }
            Self::RamOutOfRange(addr) => {
                write!(f, "Pad address 0x{:02x} is invalid", addr)
            }
            Self::StackOverflow => {
                write!(f, "CPU Stack overflow")
            }
            Self::UndefinedInstruction(op) => {
                write!(f, "Opcode {:02x}{:02x}{:02x}{:02x}", op[0], op[1], op[2], op[3])
            }
        }
    }
}

impl std::error::Error for Error {}
