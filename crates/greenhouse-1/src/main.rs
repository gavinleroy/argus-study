#[cfg(test)]
mod tests;

use brew::prelude::*;

// FIXME: I want to create a recipe that takes in `Dittany` and `Bubotuber`
// as ingredients but the resulting potion should be `Green`. I believe the
// function signature is correct, but I'm not sure why it doesn't type check.
async fn magic_brownie(i1: Dittany, i2: Bubotuber) -> Green {
  EmptyCauldron::new()
    .mix(i2)
    .boil()
    .mix(i1)
    // NOTE: I definitely want to return a `Green` potion here,
    // please don't change. We should be able to get the function
    // body to type check.
    .pour_as()
}

fn main() {
  Garden::<Wiggentree, 12>::new()
    .add_feeding_schedule(Monthly, magic_brownie)
    .garden()
}
