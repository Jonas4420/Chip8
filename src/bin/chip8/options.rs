use std::path::PathBuf;

use clap::Parser;
use sdl2::pixels::Color;

use crate::error::Error;

/// Another CHIP-8 toy emulator in Rust
#[derive(Debug, Parser)]
#[clap(name = "CHIP8")]
pub struct Options {
    /// CPU Frequency
    #[clap(long)]
    pub freq: Option<u8>,
    /// CPU PRNG seed (in hexadecimal)
    #[clap(long, parse(try_from_str = parse_u16))]
    pub seed: Option<u16>,
    /// Window background color
    #[clap(long, parse(try_from_str = parse_color))]
    pub bg: Option<Color>,
    /// Window foreground color
    #[clap(long, parse(try_from_str = parse_color))]
    pub fg: Option<Color>,
    /// Window framerate
    #[clap(long)]
    pub fps: Option<u8>,
    /// Window scale
    #[clap(long)]
    pub scale: Option<u8>,
    /// CHIP-8 ROM to run
    pub rom: PathBuf,
}

fn parse_u16(src: &str) -> Result<u16, Error> {
    if let Some(hex) = src.strip_prefix("0x") {
        Ok(u16::from_str_radix(hex, 16)?)
    } else {
        Err(Error::InvalidSeed(src.into()))
    }
}

fn parse_color(src: &str) -> Result<Color, Error> {
    if let Some(hex) = src.strip_prefix('#') {
        let hex = u32::from_str_radix(hex, 16)?;

        if hex & 0xff000000 != 0 {
            Ok(Color::RGB(
                ((hex >> 16) & 0xff) as u8,
                ((hex >> 8) & 0xff) as u8,
                (hex & 0xff) as u8,
            ))
        } else {
            Err(Error::InvalidColor(src.into()))
        }
    } else {
        Err(Error::InvalidColor(src.into()))
    }
}
