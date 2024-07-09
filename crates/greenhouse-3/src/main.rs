use brew::prelude::*;

async fn test_4(i1: Reflower, i2: Bubotuber) -> impl Potion {
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
    .add_feeding_schedule(Yearly, test_4)
    .garden()
}
