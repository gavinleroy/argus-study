#[cfg(test)]
mod tests;

use brew::prelude::*;

async fn magic_brownie(i1: Dittany, i2: Bubotuber) -> impl Potion {
  EmptyCauldron::new()
    .mix(i2)
    .boil()
    .mix(i1)
    .pour_as::<Green>()
}

fn main() {
  Garden::<Wiggentree, 12>::new()
    // FIXME: I want to create a recipe that takes in `Dittany` and `Bubotuber`
    // as ingredients but the resulting potion should be `Green`. Not
    // sure why this doesn't type check!
    .add_feeding_schedule(Monthly, magic_brownie)
    .garden()
}
