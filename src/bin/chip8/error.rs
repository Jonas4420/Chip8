use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidFramerate(u32),
    InvalidScale(usize, u8),
    InvalidScreenSize((usize, usize)),
    SdlIntegerOrError(sdl2::IntegerOrSdlError),
    SdlString(String),
    SdlWindowBuild(sdl2::video::WindowBuildError),
    UnknownMapping(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidFramerate(fps) => write!(f, "invalid framerate of {} fps", fps),
            Self::InvalidScale(x, scale) => write!(f, "cannot scale size of {} by {} times", x, scale),
            Self::InvalidScreenSize(dim) => write!(f, "screen size of {}x{} cannot be created", dim.0, dim.1),
            Self::SdlString(msg) => write!(f, "sdl: {}", msg),
            Self::SdlIntegerOrError(err) => write!(f, "sdl: {}", err),
            Self::SdlWindowBuild(err) => write!(f, "sdl: {}", err),
            Self::UnknownMapping(c) => write!(f, "char '{}' has no associated keyboard mapping", c),
        }
    }
}

impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(err: sdl2::IntegerOrSdlError) -> Self {
        Self::SdlIntegerOrError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::SdlString(err)
    }
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(err: sdl2::video::WindowBuildError) -> Self {
        Self::SdlWindowBuild(err)
    }
}

impl error::Error for Error {}
