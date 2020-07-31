use super::Train;
use super::data::{D51BODY, D51WHL, COAL};

#[derive(Copy, Clone)]
pub struct SL;

impl Train for SL {
    fn speed(&self) -> u32 {
        85
    }

    fn body(&self) -> &'static [&'static str] {
        &D51BODY
    }

    fn wheelset(&self, x: usize) -> &'static [&'static str] {
        &D51WHL[(x % 6)]
    }

    fn tender(&self) -> Option<&'static [&'static str]> {
        Some(&COAL)
    }
}
