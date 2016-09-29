use super::Train;
use super::data::{LOGO, LWHL, LCOAL, LCAR};

#[derive(Copy, Clone)]
pub struct Logo;

impl Train for Logo {
    fn body(&self) -> &'static [&'static str] {
        &LOGO
    }

    fn wheelset(&self, x: usize) -> &'static [&'static str] {
        &LWHL[(x % 6)]
    }

    fn tender(&self) -> Option<&'static [&'static str]> {
        Some(&LCOAL)
    }

    fn wagons(&self) -> u32 {
        2
    }

    fn wagon(&self) -> Option<&'static [&'static str]> {
        Some(&LCAR)
    }
}
