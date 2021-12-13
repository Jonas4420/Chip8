use std::path::Path;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::error::Error;
use crate::ram::Ram;
use crate::rng::Rng;
use crate::screen::Screen;
use crate::timer::Timer;

const PAD_MAPPINGS: [(char, usize); 0x10] = [
    ('1', 0x1),
    ('2', 0x2),
    ('3', 0x3),
    ('4', 0xc),
    ('q', 0x4),
    ('w', 0x5),
    ('e', 0x6),
    ('r', 0xd),
    ('a', 0x7),
    ('s', 0x8),
    ('d', 0x9),
    ('f', 0xe),
    ('z', 0xa),
    ('x', 0x0),
    ('c', 0xb),
    ('v', 0xf),
];
const SCREEN_SIZE: (usize, usize) = (64, 32);

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    rng: Rng,
    dt: Timer,
    st: Timer,
    mapping: [char; PAD_MAPPINGS.len()],
    screen_size: (usize, usize),
}

impl<'a> Chip8 {
    pub fn new() -> Self {
        let mut sorted_map: Vec<_> = PAD_MAPPINGS.into();
        sorted_map.sort_by_key(|mapping| mapping.1);

        let mut mapping = [Default::default(); PAD_MAPPINGS.len()];
        mapping
            .iter_mut()
            .zip(sorted_map.into_iter().map(|(key, _)| key))
            .for_each(|(dst, src)| *dst = src);

        Self {
            cpu: Default::default(),
            ram: Default::default(),
            rng: Default::default(),
            dt: Default::default(),
            st: Default::default(),
            mapping,
            screen_size: SCREEN_SIZE,
        }
    }

    pub fn load_rom(&mut self, rom: &Path) -> Result<(), Error> {
        // TODO: open file
        // TODO: read file
        // TODO: write file in ram
        // TODO: write font in ram
        // TODO: setup PC
        // TODO: setup font offset
        // TODO: setup seed

        // self.cpu.init(pc, ft);
        // self.rng.seed();

        Ok(())
    }

    pub fn clock(&mut self, screen: &mut [bool], pad: &[bool], buzz: &mut bool) -> Result<(), Error> {
        if screen.len() != (self.screen_size.0 * self.screen_size.1) {
            return Err(Error::InvalidScreenSize(
                self.screen_size.0 * self.screen_size.1,
                screen.len(),
            ));
        }

        if pad.len() != self.mapping.len() {
            return Err(Error::InvalidPadSize(self.mapping.len(), pad.len()));
        }

        let mut bus = Bus {
            ram: &mut self.ram,
            rng: &mut self.rng,
            dt: &mut self.dt,
            st: &mut self.st,
            screen: Screen {
                memory: screen,
                size: self.screen_size,
            },
            pad,
        };

        // TODO: clock at correct frequency
        self.cpu.cycle(&mut bus)?;
        self.dt.clock();
        self.st.clock();

        *buzz = self.st.get() > 0;

        Ok(())
    }

    pub fn get_mapping(&self) -> &[char] {
        &self.mapping
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        self.screen_size
    }
}
