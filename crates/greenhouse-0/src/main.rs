#[cfg(test)]
mod tests;

use brew::prelude::*;
use brew_macros::HumanFood;

#[derive(HumanFood)]
struct Meatball;

#[derive(HumanFood)]
struct Tomato;

#[derive(HumanFood)]
struct Pasta;

async fn make_spaghetti(
  i1: Liquified<Tomato>,
  i2: Meatball,
  i3: Pasta,
) -> impl Remedy {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .mix(i3)
    .pour_as::<Blue>()
}

fn main() {
  Garden::<Alihotsy, 1>::new()
    // FIXME: I'm trying to make a recipe for spaghetti. I know that there needs
    // to be Tomato, Meatball, and Pasta. Overall the recipe should be a remedy,
    // but it doesn't type check and I'm not sure why.
    .add_feeding_schedule(Daily, make_spaghetti)
    .garden()
}
