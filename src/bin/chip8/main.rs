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
    let mut chip8 = Chip8::new();
    let mut window = Window::new(chip8.get_screen_size(), chip8.get_key_mapping(), &options)?;

    chip8.load_rom(&options.rom)?;

    window.run(|pad, screen, audio| {
        chip8.clock(screen, pad, audio)?;
        Ok(())
    })?;

    Ok(())
}
