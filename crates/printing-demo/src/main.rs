//! Sample code for using traits to define shared behavior.
//!
//! Code taken from the rustbook chapter 10, "Traits: Defining Shared Behavior".
use std::fmt::{self, Display, Debug};

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


impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}


impl<T: Eq> Eq for Pair<T> {}
impl<T: PartialEq> PartialEq for Pair<T> {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}


fn main() {
  let p0 = Pair::new(1, 2);
  p0.cmp_display();

  let p1 = Pair::new("1", "2");
  p1.cmp_display();

  let p2 = Pair::new(1.2, 3.14);
  p2.cmp_display();

  let mut points = vec![
    Pair::new(1, 2),
    Pair::new(1, 2),
    Pair::new(2, 1),
  ];

  points.sort_unstable();
  println!("{:?}", points);
}
