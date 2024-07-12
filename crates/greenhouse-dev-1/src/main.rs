//! Development Task: finish TODO items and make the tests pass.
#![allow(unused_imports)]

#[cfg(test)]
mod tests;

use brew::prelude::*;
use brew_macros::*;

// TODO: make a new recipe `dirty_cake` that combines the following ingredients
// in this order to make a `Poison`.
//
// - Shrivelfig
// - Wiggentree
//
// Additionally, your recipe must boil the ingredients at least once.

fn main() {
  Garden::<Dittany, 1024>::new()
    .add_feeding_schedule(Daily, dirty_cake)
    .garden()
}
