use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
    InvalidColor(String),
    InvalidSeed(String),
    ParseInt(num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidColor(color) => {
                write!(f, "invalid color '{}', expected format is #RRGGBB", color)
            }
            Self::InvalidSeed(seed) => {
                write!(f, "invalid seed '{}', expected format is 0xXXXX", seed)
            }
            Self::ParseInt(err) => err.fmt(f),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl error::Error for Error {}
