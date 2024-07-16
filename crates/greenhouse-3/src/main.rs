#[cfg(test)]
mod tests;

use brew::prelude::*;

async fn fertilize(i1: Reflower, i2: Bubotuber) -> impl Potion {
  EmptyCauldron::new()
    .mix(i1)
    .boil()
    .mix(i2)
    .rest()
    .rest()
    .pour_as::<Blue>()
}

fn main() {
  Garden::<Alihotsy, 2>::new()
    // FIXME: I'm trying to make recipe of two ingredients that returns a `Blue`
    // potion, but I can't get this to type check.
    .add_feeding_schedule(Yearly, fertilize)
    .garden()
}
