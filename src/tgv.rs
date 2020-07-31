use super::Train;
use super::data::{TGVWHL, TGVBODY, TGVCAR};

#[derive(Copy, Clone)]
pub struct TGV;

impl Train for TGV {
    fn speed(&self) -> u32 {
        // in reality, TGV is even faster, but we want to be able to see it!
        300
    }

    fn body(&self) -> &'static [&'static str] {
        &TGVBODY
    }

    fn wheelset(&self, x: usize) -> &'static [&'static str] {
        &TGVWHL[(x % 2)]
    }

    fn wagons(&self) -> u32 {
        2
    }

    fn wagon(&self) -> Option<&'static [&'static str]> {
        Some(&TGVCAR)
    }
}
