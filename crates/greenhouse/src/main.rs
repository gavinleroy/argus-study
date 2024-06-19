//! A safe space to cultivate exotic plants.

use brew::prelude::*;
use brew_macros::Food;

fn test_0() {
  #[derive(Food)]
  struct Pizza;

  async fn human_food(i1: Pizza) -> impl Potion {
    EmptyCauldron::default().mix(i1).pour_as::<Blue>()
  }

  Garden::<Alihotsy, 1>::new()
    .add_feeding_schedule(Daily, human_food)
    .garden()
}

fn test_1() {
  async fn brighten_leaves(
    i1: Dittany,
    i2: Dittany,
    i3: Aconite,
  ) -> impl Potion {
    EmptyCauldron::default()
      .mix(i1)
      .mix(i2)
      .mix(i3)
      .pour_as::<Blue>()
  }

  Garden::<Bubotuber, 3>::new()
    .add_feeding_schedule(Daily, brighten_leaves)
    .garden()
}

fn use_test_2() {
  async fn test_2(i1: Dittany, i2: Bubotuber) -> impl Remedy {
    EmptyCauldron::default()
      .mix(i1)
      .boil()
      .mix(i2)
      .pour_as::<Green>()
  }

  Garden::<Wiggentree, 12>::new()
    .add_feeding_schedule(Monthly, test_2)
    .garden()
}

fn test_3(the_neighbors_figs: &mut Garden<Shrivelfig, 6>) {
  async fn shrivel(i1: Dittany) -> impl Potion {
    EmptyCauldron::default()
      .mix(i1)
      .boil()
      .rest()
      .pour_as::<Pink>()
  }

  the_neighbors_figs.sabotage(shrivel);
}

fn use_test_4() {
  async fn test_4(i1: Dittany, i2: Bubotuber) -> impl Potion {
    EmptyCauldron::default().mix(i1).mix(i2).pour_as::<Blue>()
  }

  Garden::<Alihotsy, 2>::new()
    .add_feeding_schedule(Yearly, test_4)
    .garden()
}

fn test_5(gardens: &Vec<&dyn AsGarden>) {
  async fn speed_feed(
    i1: Dittany,
    i2: Shrivelfig,
    i3: Reflower,
  ) -> impl Potion {
    EmptyCauldron::default()
      .mix(i1)
      .mix(i2)
      .mix(i3)
      .pour_as::<Blue>()
  }

  gardens.feed_in_parallel(speed_feed);
}

fn main() {
  test_1();
  use_test_2();
  test_3(&mut Garden::new());
  use_test_4();
  test_5(&Default::default());
}
