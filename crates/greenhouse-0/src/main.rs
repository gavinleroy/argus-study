#[cfg(test)]
mod tests;

use brew::prelude::*;

// NOTE: The documentation said to mark these as `Mineral`s so that I
// can mix them into a recipe. I'm not sure entirely what that means.
#[derive(Mineral)]
struct Nitrogen;

#[derive(Mineral)]
struct Phosphorus;

async fn recipe(i1: Nitrogen, i2: Phosphorus, i3: Alihotsy) -> Blue {
  // NOTE: I definitely want to return a blue potion, you
  // shouldn't need to change the function body.
  EmptyCauldron::new().mix(i1).mix(i2).mix(i3).pour_as()
}

fn main() {
  Garden::<Wiggentree, 1>::new()
    // FIXME: I'm trying to make a hearty recipe for my plants that contains
    // `Phosphorus` and `Nitrogen`, key ingredients in fertilizer. It should
    // also contain `Alihotsy` to help with leaf growth.
    .add_feeding_schedule(Daily, recipe)
    .garden()
}
