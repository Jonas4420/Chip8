use std::path::PathBuf;
use std::{error, fmt, num};

use clap::Parser;
use sdl2::pixels::Color;

/// Another CHIP-8 toy emulator in Rust
#[derive(Debug, Parser)]
#[clap(name = "CHIP8")]
pub struct Options {
    /// Window background color (format: #RRGGBB)
    #[clap(long, parse(try_from_str = parse_color))]
    pub bg: Option<Color>,
    /// Window foreground color (format: #RRGGBB)
    #[clap(long, parse(try_from_str = parse_color))]
    pub fg: Option<Color>,
    /// Window framerate
    #[clap(long)]
    pub fps: Option<u32>,
    /// CPU Frequency (in hertz)
    #[clap(long)]
    pub freq: Option<f32>,
    /// Window scale
    #[clap(long, possible_values = [ "1", "2", "4", "8", "16" ])]
    pub scale: Option<u8>,
    /// CPU PRNG seed (in hexadecimal)
    #[clap(long, parse(try_from_str = parse_u16))]
    pub seed: Option<u16>,
    /// Path to CHIP-8 ROM to run
    pub rom: PathBuf,
}

#[derive(Debug)]
pub enum OptionError {
    InvalidColor(String),
    InvalidSeed(String),
    ParseInt(num::ParseIntError),
}

fn parse_u16(src: &str) -> Result<u16, OptionError> {
    if let Some(hex) = src.strip_prefix("0x") {
        Ok(u16::from_str_radix(hex, 16)?)
    } else {
        Err(OptionError::InvalidSeed(src.into()))
    }
}

fn parse_color(src: &str) -> Result<Color, OptionError> {
    if let Some(hex) = src.strip_prefix('#') {
        let hex = u32::from_str_radix(hex, 16)?;

        if hex & 0xff000000 != 0 {
            Ok(Color::RGB(
                ((hex >> 16) & 0xff) as u8,
                ((hex >> 8) & 0xff) as u8,
                (hex & 0xff) as u8,
            ))
        } else {
            Err(OptionError::InvalidColor(src.into()))
        }
    } else {
        Err(OptionError::InvalidColor(src.into()))
    }
}

impl fmt::Display for OptionError {
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

impl From<num::ParseIntError> for OptionError {
    fn from(err: num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl error::Error for OptionError {}
