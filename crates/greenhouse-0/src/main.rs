use brew::prelude::*;
use brew_macros::HumanFood;

#[derive(HumanFood)]
struct Meatball;

#[derive(HumanFood)]
struct Tomatoe;

#[derive(HumanFood)]
struct Pasta;

async fn make_spaghetti(
  i1: Liquified<Tomatoe>,
  i2: Meatball,
  i3: Pasta,
) -> impl Potion {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .mix(i3)
    .pour_as::<Blue>()
}

fn main() {
  Garden::<Alihotsy, 1>::new()
    .add_feeding_schedule(Daily, make_spaghetti)
    .garden()
}
