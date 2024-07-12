//! Development Task: finish TODO items and make the tests pass.
#![allow(unused_imports)]

#[cfg(test)]
mod tests;

use brew::prelude::*;
use brew_macros::*;


struct Mozerella;

// TODO: make a new recipe `gloop` that combines the following ingredients
// in this order to make a `Remedy`.
//
// - Reflower
// - Mozerella
// - Alihotsy
// - Shrivelfig

fn main() {
  Garden::<Alihotsy, 1>::new()
    .add_feeding_schedule(Yearly, gloop)
    .garden()
}
