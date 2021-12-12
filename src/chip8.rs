use std::path::Path;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::error::Error;
use crate::ram::Ram;
use crate::timer::Timer;

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    dt: Timer,
    st: Timer,
    dim: (usize, usize),
    mappings: [char; 16],
}

impl<'a> Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Default::default(),
            ram: Default::default(),
            dt: Default::default(),
            st: Default::default(),
            dim: (64, 32),
            mappings: [
                'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 'z', 'c', '4', 'r', 'f', 'v',
            ],
        }
    }

    pub fn load_rom(&mut self, rom: &Path) -> Result<(), Error> {
        Ok(())
    }

    pub fn clock(&mut self, screen: &mut [bool], pad: &[bool], buzz: &mut bool) -> Result<(), Error> {
        let mut bus = Bus {
            screen: screen,
            pad: pad,
            ram: &mut self.ram,
            dt: &mut self.dt,
            st: &mut self.st,
        };

        self.cpu.cycle(&mut bus)?;

        *buzz = self.st.get() > 0;

        Ok(())
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        self.dim
    }

    pub fn get_key_mapping(&self) -> &[char] {
        &self.mappings
    }
}
