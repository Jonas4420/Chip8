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
