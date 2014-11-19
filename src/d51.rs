use super::Train;
use super::data::{D51BODY, D51WHL, COAL};

pub struct SL;

impl Train for SL {
  fn body(&self) -> &'static [&'static str] {
    D51BODY
  }

  fn wheelset(&self, x: uint) -> &'static [&'static str] {
    D51WHL[x % 6]
  }

  fn tender(&self) -> Option<&'static [&'static str]> {
    Some(COAL.as_slice())
  }
}

