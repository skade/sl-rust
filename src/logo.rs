use super::Train;
use super::data::{LOGO, LWHL, LCOAL, LCAR};

pub struct Logo;

impl Train for Logo {
  fn body(&self) -> &'static [&'static str] {
    &LOGO
  }

  fn wheelset(&self, x: uint) -> &'static [&'static str] {
    &LWHL[x % 6]
  }

  fn tender(&self) -> Option<&'static [&'static str]> {
    Some(LCOAL.as_slice())
  }

  fn wagons(&self) -> uint {
    2
  }

  fn wagon(&self) -> Option<&'static [&'static str]> {
    Some(LCAR.as_slice())
  }
}
