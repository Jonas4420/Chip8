use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidFps(u32),
    ScaleOverflow(usize, u8),
    ScreenTooLarge((usize, usize)),
    Sdl(SdlError),
    UnknownMapping(char),
}

#[derive(Debug)]
pub enum SdlError {
    IntegerOrError(sdl2::IntegerOrSdlError),
    String(String),
    WindowBuildError(sdl2::video::WindowBuildError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidFps(fps) => write!(f, "framerate of {} fps is invalid", fps),
            Self::ScaleOverflow(x, scale) => write!(f, "cannot scale size of {} by {} times", x, scale),
            Self::ScreenTooLarge(dim) => write!(f, "screen size of {}x{} cannot be created", dim.0, dim.1),
            Self::Sdl(err) => match err {
                SdlError::IntegerOrError(err) => err.fmt(f),
                SdlError::String(err) => err.fmt(f),
                SdlError::WindowBuildError(err) => err.fmt(f),
            },
            Self::UnknownMapping(c) => write!(f, "char '{}' has no associated keyboard mapping", c),
        }
    }
}

impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(err: sdl2::IntegerOrSdlError) -> Self {
        Self::Sdl(SdlError::IntegerOrError(err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Sdl(SdlError::String(err))
    }
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(err: sdl2::video::WindowBuildError) -> Self {
        Self::Sdl(SdlError::WindowBuildError(err))
    }
}

impl error::Error for Error {}
