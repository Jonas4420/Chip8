use crate::ram::Ram;
use crate::timer::Timer;

#[derive(Debug)]
pub struct Bus<'a> {
    pub screen: &'a mut [bool],
    pub pad: &'a [bool],
    pub ram: &'a mut Ram,
    pub dt: &'a mut Timer,
    pub st: &'a mut Timer,
}
