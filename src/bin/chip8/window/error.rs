use std::error;
use std::fmt;

// TODO: refactor

#[derive(Debug)]
pub enum WindowError {
    SdlStr(String),
    SdlErr(sdl2::IntegerOrSdlError),
    SdlWin(sdl2::video::WindowBuildError),
    UnknownMapping(char),
    InvalidScreenSize(usize, usize),
    InvalidScale(usize, u8),
    InvalidFPS(u32),
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SdlStr(msg) => write!(f, "{}", msg),
            Self::SdlErr(err) => err.fmt(f),
            Self::SdlWin(err) => err.fmt(f),
            Self::UnknownMapping(c) => write!(f, "{}", c),
            Self::InvalidScreenSize(width, height) => {
                write!(f, "{}{}", width, height)
            }
            Self::InvalidScale(x, scale) => {
                write!(f, "{}{}", x, scale)
            }
            Self::InvalidFPS(fps) => {
                write!(f, "{}", fps)
            }
        }
    }
}

impl From<String> for WindowError {
    fn from(err: String) -> Self {
        Self::SdlStr(err)
    }
}

impl From<sdl2::IntegerOrSdlError> for WindowError {
    fn from(err: sdl2::IntegerOrSdlError) -> Self {
        Self::SdlErr(err)
    }
}

impl From<sdl2::video::WindowBuildError> for WindowError {
    fn from(err: sdl2::video::WindowBuildError) -> Self {
        Self::SdlWin(err)
    }
}

impl error::Error for WindowError {}
