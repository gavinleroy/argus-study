use std::fmt;

pub fn in_pink<T: fmt::Display>(v: &T) {
  println!("\x1b[0;35m{v}\x1b[0m");
}
