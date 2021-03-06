#[derive(Debug, Default)]
pub struct Rng {
    state: u16,
}

impl Rng {
    pub fn seed(&mut self, seed: u16) {
        self.state = seed;

        // Filter out first 16 bits, to not directly output the seed
        for _ in 0..16 {
            self.next();
        }
    }

    pub fn get_byte(&mut self) -> u8 {
        (0..8).fold(0x00, |acc, _| {
            let bit = (self.state & 0x0001) as u8;
            self.next();
            (acc << 1) | bit
        })
    }

    pub fn next(&mut self) {
        // P(X) = X^16 + X^15 + X^13 + X^4 + 1
        let bit = (self.state ^ (self.state >> 1) ^ (self.state >> 3) ^ (self.state >> 12)) & 0x1;
        self.state = (self.state >> 1) | (bit << 15);
    }
}
