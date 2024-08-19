#[cfg(test)]
mod tests;

use brew::prelude::*;

#[derive(Mineral)]
struct Nitrogen;

// NOTE: The recipe calls for a `Nitrogen`-based fertilizer and
// `Reflower`. The signature shouldn't need to be tweaked.
async fn supplement(i1: Reflower, i2: Fertilizer<Nitrogen>) -> Blue {
  EmptyCauldron::new()
    .mix(i1)
    .boil()
    .mix(i2)
    .rest()
    .rest()
    // NOTE: I definitely want to pour a `Blue` potion here, don't change this line.
    .pour_as()
}

fn main() {
  Garden::<Alihotsy, 2>::new()
    // FIXME: I'm trying to make recipe of two ingredients that returns
    // a `Blue` potion, but I can't get this to type check.
    .add_feeding_schedule(Yearly, supplement)
    .garden()
}
