use clap::Parser;

use options::Options;
use window::WindowBuilder;

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

    let screen_size = (64, 32);

    let mappings = [
        'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 'z', 'c', '4', 'r', 'f', 'v',
    ];
    let window = WindowBuilder::new(screen_size, &mappings, &options)?;
    let mut window = window.present();

    while window.is_open() {
        window.process_events()?;

        // TODO: clock CHIP-8
        let (pad, screen) = window.get_io();

        // let screen = &mut window.video;
        for x in 0..screen_size.0 {
            for y in 0..screen_size.1 {
                screen[(y * screen_size.0) + x] = (x == 10) ^ pad[1];
            }
        }

        let sound_timer = if pad[2] { 1 } else { 0 };

        window.render(sound_timer)?;

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    Ok(())
}
