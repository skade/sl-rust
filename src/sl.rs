pub mod d51;
pub mod c51;
pub mod logo;
mod data;

pub trait Train {
    fn body(&self) -> &'static [&'static str];
    fn wheelset(&self, x: u32) -> &'static [&'static str];
    fn tender(&self) -> Option<&'static [&'static str]> {
        None
    }
    fn wagons(&self) -> u32 {
        0
    }
    fn wagon(&self) -> Option<&'static [&'static str]> {
        None
    }
}
