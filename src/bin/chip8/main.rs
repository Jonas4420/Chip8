use clap::Parser;

use options::Options;
use window::Window;

use chip8::Chip8;

mod error;
mod options;
mod window;

fn main() {
    try_main().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse_from(std::env::args());
    let rom = std::fs::read(&options.rom)?;

    let mut chip8 = Chip8::new(options.freq);
    let mut window = Window::new(chip8.get_screen_size(), chip8.get_pad_map(), &options)?;

    chip8.load_rom(&rom, options.seed)?;

    window.run(|io| {
        chip8.clock(io)?;
        Ok(())
    })?;

    Ok(())
}
