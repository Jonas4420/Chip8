#[derive(Debug)]
pub struct Screen<'a> {
    pub memory: &'a mut [bool],
    pub size: (usize, usize),
}

impl<'a> Screen<'a> {
    pub fn clear(&mut self) {
        for px in self.memory.iter_mut() {
            *px = false;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, byte: u8) -> bool {
        // TODO: refactor
        let x = (x as usize) % self.size.0;
        let y = (y as usize) % self.size.1;

        let line_start = y * self.size.0;
        let line_end = line_start + self.size.0;
        let line = &mut self.memory[line_start..line_end];

        let mut erased = false;
        for i in 0..8 {
            let x_mod = (x.wrapping_add(i)) % line.len();
            let px = ((byte << i) & 0x80) != 0x00;

            let old = line[x_mod];

            line[x_mod] = old ^ px;
            erased = erased | (old & px);
        }

        erased
    }
}
