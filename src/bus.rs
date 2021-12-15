use crate::screen;

mod ram;
mod rng;
mod timer;

#[derive(Debug, Default)]
pub struct Bus {
    pub ram: ram::Ram,
    pub rng: rng::Rng,
    pub dt: timer::Timer,
    pub st: timer::Timer,
}

#[derive(Debug)]
pub struct IO<'a> {
    pub screen: &'a mut dyn screen::Screen,
    pub pad: &'a [bool],
    pub audio: &'a mut bool,
}
