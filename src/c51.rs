use super::Train;
use super::data::{C51WHL, C51BODY, COAL};

pub struct C51;

impl Train for C51 {
  fn body(&self) -> &'static [&'static str] {
    C51BODY
  }

  fn wheelset(&self, x: uint) -> &'static [&'static str] {
    C51WHL[x % 6]
  }

  fn tender(&self) -> Option<&'static [&'static str]> {
    Some(COAL.as_slice())
  }
}
