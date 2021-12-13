use crate::error::Error;

#[derive(Debug)]
pub struct Rng {
    seed: u16,
}

impl Rng {
    pub fn new(seed: u16) -> Result<Self, Error> {
        if seed != 0x0000 {
            Ok(Self { seed })
        } else {
            Err(Error::RngSeedNul)
        }
    }

    pub fn get_byte(&mut self) -> u8 {
        // TODO
        (self.seed & 0xff) as u8
    }
}
