use std::fmt;
use std::io;

// TODO: refactor errors

#[derive(Debug)]
pub enum Error {
    InvalidScreenSize(usize, usize),
    InvalidPadSize(usize, usize),
    InvalidAddress(u16),
    UnknownOpcode([u8; 4]),
    PadAddressOutOfRange(u8),
    StackOverflow,
    RngSeedNul,
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "")
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl std::error::Error for Error {}
