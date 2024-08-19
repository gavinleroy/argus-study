#[cfg(test)]
mod tests;

use brew::prelude::*;
use rayon::prelude::*;

// NOTE: The recipe calls for `Dittany`, `Shrivelfig`, and `Reflower` to be mixed
// into the cauldron. The signature may need to be tweaked to make this recipe work.
async fn fertilize(i1: Dittany, i2: Shrivelfig, i3: Reflower) -> Blue {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .mix(i3)
    // NOTE: I definitely want to pour the result as
    // a `Blue` potions, this shouldn't change.
    .pour_as()
}

fn main() {
  let mut gardens = vec![];
  gardens.resize_with(2048, || Garden::<Alihotsy, 4>::new());

  gardens.par_chunks(32).for_each(|gardens| {
    // FIXME: I want to make a recipe of three ingredients that returns a
    // `Blue` potion, but I can't figure out why this doesn't type check.
    gardens.feed_in_parallel(fertilize)
  })
}
