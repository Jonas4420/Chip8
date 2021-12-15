use std::fmt;

pub trait Screen {
    fn as_slice(&self) -> &[bool];
    fn as_mut_slice(&mut self) -> &mut [bool];
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn clear(&mut self) {
        for px in self.as_mut_slice() {
            *px = false;
        }
    }

    fn draw(&mut self, x: u8, y: u8, byte: u8) -> bool {
        let width = self.get_width();
        let height = self.get_height();

        let line = self
            .as_mut_slice()
            .chunks_mut(width)
            .nth((y as usize) % height)
            .unwrap();

        (0..8).fold(false, |acc, i| {
            let x = ((x as usize).wrapping_add(i)) % width;
            let px = ((byte << i) & 0x80) != 0x00;

            let erased = line[x] & px;
            line[x] ^= px;
            acc | erased
        })
    }

    fn size(&self) -> (usize, usize) {
        (self.get_width(), self.get_height())
    }
}

impl fmt::Debug for &mut dyn Screen {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("dyn Screen")
            .field("memory", &self.as_slice())
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .finish()
    }
}
