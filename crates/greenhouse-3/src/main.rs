use brew::prelude::*;

async fn test_4(i1: Dittany, i2: Bubotuber) -> impl Potion {
  EmptyCauldron::new().mix(i1).mix(i2).pour_as::<Yellow>()
}

fn main() {
  Garden::<Alihotsy, 2>::new()
    .add_feeding_schedule(Yearly, test_4)
    .garden()
}
