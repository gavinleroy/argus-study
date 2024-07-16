#[cfg(test)]
mod tests;

use brew::prelude::*;
use rayon::prelude::*;

async fn speed_feed(i1: Dittany, i2: Shrivelfig, i3: Reflower) -> impl Potion {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .mix(i3)
    .pour_as::<Blue>()
}

fn main() {
  let mut gardens = vec![];
  gardens.resize_with(2048, || Garden::<Alihotsy, 4>::new());

  gardens
    .par_chunks(32)
    // FIXME: I want to make a recipe of three ingredients that returns a
    // `Blue` potion, but I can't figure out why this doesn't type check.
    .for_each(|gardens| gardens.feed_in_parallel(speed_feed))
}
