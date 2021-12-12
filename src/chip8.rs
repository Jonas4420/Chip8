use std::path::Path;

#[derive(Debug)]
pub struct Chip8 {
    width: usize,
    height: usize,
    st: u8,
    mappings: [char; 16],
}

impl Chip8 {
    pub fn load_rom(rom: &Path) -> Self {
        Self {
            width: 64,
            height: 32,
            st: 0,
            mappings: [
                'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 'z', 'c', '4', 'r', 'f', 'v',
            ],
        }
    }

    pub fn clock(&mut self, screen: &mut [bool], pad: &[bool], audio: &mut bool) -> Result<(), String> {
        for x in 0..self.width {
            for y in 0..self.height {
                screen[(y * self.width) + x] = (x == y) ^ pad[1];
            }
        }

        if pad[3] {
            return Err("OOPS".into());
        }

        self.st = if pad[2] { 1 } else { 0 };
        *audio = self.st > 0;

        Ok(())
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_key_mapping(&self) -> &[char] {
        &self.mappings
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.st
    }
}
