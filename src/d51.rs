use super::Train;
use super::data::{D51BODY, D51WHL, COAL};

#[derive(Copy, Clone)]
pub struct SL;

impl Train for SL {
    fn body(&self) -> &'static [&'static str] {
        &D51BODY
    }

    fn wheelset(&self, x: u32) -> &'static [&'static str] {
        &D51WHL[(x % 6) as usize]
    }

    fn tender(&self) -> Option<&'static [&'static str]> {
        Some(&COAL)
    }
}
