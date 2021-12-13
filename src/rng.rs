use crate::error::Error;

#[derive(Debug, Default)]
pub struct Rng {
    seed: u16,
}

impl Rng {
    pub fn seed(&mut self, seed: u16) -> Result<(), Error> {
        if seed != 0x0000 {
            self.seed = seed;
            Ok(())
        } else {
            Err(Error::RngSeedNul)
        }
    }

    pub fn get_byte(&mut self) -> u8 {
        // TODO
        (self.seed & 0xff) as u8
    }
}
