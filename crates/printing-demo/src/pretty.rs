use std::fmt::Debug;

pub trait InPink: Debug {}
impl<T: Debug> InPink for T {}

pub fn inpink<T: InPink>(v: T) {
  println!("\x1b[0;35m{v:?}\x1b[0m");
}
