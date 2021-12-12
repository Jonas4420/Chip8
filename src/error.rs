use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidScreenSize(usize, usize),
    InvalidPadSize(usize, usize),
    InvalidAddress(u16),
    UnknownOpcode([u8; 4]),
    PadAddressOutOfRange(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "")
    }
}

impl std::error::Error for Error {}
