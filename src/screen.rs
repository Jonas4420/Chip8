#[derive(Debug)]
pub struct Screen<'a> {
    pub memory: &'a mut [bool],
    pub width: usize,
    pub height: usize,
}

impl<'a> Screen<'a> {
    pub fn clear(&mut self) {
        for px in self.memory.iter_mut() {
            *px = false;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, byte: u8) -> bool {
        let line = self
            .memory
            .chunks_mut(self.width)
            .nth((y as usize) % self.height)
            .unwrap();

        (0..8).fold(false, |acc, i| {
            let x = ((x as usize).wrapping_add(i)) % self.width;
            let px = ((byte << i) & 0x80) != 0x00;

            let erased = line[x] & px;
            line[x] ^= px;
            acc | erased
        })
    }
}
