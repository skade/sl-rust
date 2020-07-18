pub mod d51;
pub mod c51;
pub mod logo;
pub mod tgv;
mod data;

pub trait Train {
    /// Approximate speed in km/h
    fn speed(&self) -> u32 {
        100
    }
    fn body(&self) -> &'static [&'static str];
    fn wheelset(&self, x: usize) -> &'static [&'static str];
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
