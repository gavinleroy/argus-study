use std::fmt;

#[derive(Clone, Debug)]
pub struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: fmt::Display> fmt::Display for Pair<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

pub type Name<'a> = Pair<&'a str>;
