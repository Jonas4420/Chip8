use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidScreenSize(usize, usize),
    InvalidPadSize(usize, usize),
    InvalidAddress(u16),
    UnknownOpcode([u8; 4]),
    PadAddressOutOfRange(u8),
    StackOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for Error {}
