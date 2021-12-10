use clap::Parser;

mod error;
mod options;

fn main() {
    try_main().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = options::Options::parse_from(std::env::args());

    // println!("{:#?}", args);
    Ok(())
}
