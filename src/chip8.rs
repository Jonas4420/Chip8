use std::fs;
use std::io::prelude::*;
use std::path;

use crate::bus::Bus;
use crate::clock::Clock;
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
const FONT_SPRITES: [[u8; 5]; 0x10] = [
    [0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000],
    [0b00100000, 0b01100000, 0b00100000, 0b00100000, 0b01110000],
    [0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000],
    [0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000],
    [0b10010000, 0b10010000, 0b11110000, 0b00010000, 0b00010000],
    [0b11110000, 0b10000000, 0b11110000, 0b00010000, 0b11110000],
    [0b11110000, 0b10000000, 0b11110000, 0b10010000, 0b11110000],
    [0b11110000, 0b00010000, 0b00100000, 0b01000000, 0b01000000],
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000],
    [0b11110000, 0b10010000, 0b11110000, 0b00010000, 0b11110000],
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b10010000],
    [0b11100000, 0b10010000, 0b11100000, 0b10010000, 0b11100000],
    [0b11110000, 0b10000000, 0b10000000, 0b10000000, 0b11110000],
    [0b11100000, 0b10010000, 0b10010000, 0b10010000, 0b11100000],
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b11110000],
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b10000000],
];
const SCREEN_SIZE: (usize, usize) = (64, 32);
const PROGRAM_START: u16 = 0x0200;
const FONT_OFFSET: u16 = 0x0000;
const RNG_SEED: u16 = 0xcafe;

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    rng: Rng,
    dt: Timer,
    st: Timer,
    mapping: [char; PAD_MAPPINGS.len()],
    screen_size: (usize, usize),
    clock_60htz: Clock,
    clock_cpu: Clock,
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
            clock_60htz: Clock::new(std::time::Duration::from_secs(1).div_f32(60.0)),
            clock_cpu: Clock::new(std::time::Duration::from_secs(1).div_f32(500.0)),
        }
    }

    pub fn load_rom<T>(&mut self, rom: T, freq: Option<f32>, seed: Option<u16>) -> Result<(), Error>
    where
        T: AsRef<path::Path>,
    {
        // TODO: Don't do IO here
        let mut f = fs::File::open(rom)?;
        let pc = PROGRAM_START;
        let ft = FONT_OFFSET;
        let seed = seed.unwrap_or(RNG_SEED);

        for (i, byte) in f.bytes().enumerate() {
            let addr = pc.wrapping_add(i as u16);
            self.ram.write(addr, byte?)?;
        }

        let mut addr = ft;
        for sprite in FONT_SPRITES {
            for (i, byte) in sprite.iter().enumerate() {
                self.ram.write(addr, *byte)?;
                addr = addr.wrapping_add(1);
            }
        }

        self.cpu.init(pc, ft);
        self.rng.seed(seed)?;

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
        self.clock_cpu
            .tick(std::time::Instant::now(), || self.cpu.cycle(&mut bus))?;

        self.clock_60htz.tick(std::time::Instant::now(), || {
            self.dt.clock();
            self.st.clock();
            Ok(())
        });

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
