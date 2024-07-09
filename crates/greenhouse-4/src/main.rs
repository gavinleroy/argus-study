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
    .for_each(|gardens| gardens.feed_in_parallel(speed_feed))
}
