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
        // TODO
        unimplemented!()
    }
}
