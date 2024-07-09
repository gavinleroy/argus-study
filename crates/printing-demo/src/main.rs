//! Sample code for using traits to define shared behavior.
//!
//! Code taken from the rustbook chapter 10, "Traits: Defining Shared Behavior".
mod pretty;

use std::fmt::{self, Debug};

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Debug> Debug for Pair<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Pair {{ x: {:?}, y: {:?} }}", self.x, self.y)
  }
}

struct Name<'a>(Pair<&'a str>);

impl Name<'_> {
  fn new<'a>(first: &'a str, last: &'a str) -> Name<'a> {
    Name(Pair::new(first, last))
  }
}

fn main() {
  let names = vec![
    Name::new("albert", "einstein"),
    Name::new("marie", "curie"),
    Name::new("jane", "goodall"),
  ];

  pretty::inpink(&names);
}
