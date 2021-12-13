use crate::ram::Ram;
use crate::rng::Rng;
use crate::screen::Screen;
use crate::timer::Timer;

#[derive(Debug)]
pub struct Bus<'a> {
    pub ram: &'a mut Ram,
    pub rng: &'a mut Rng,
    pub dt: &'a mut Timer,
    pub st: &'a mut Timer,
    pub screen: Screen<'a>,
    pub pad: &'a [bool],
}
