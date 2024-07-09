use brew::prelude::*;

async fn magic_brownie(i1: Dittany, i2: Bubotuber) -> impl Remedy {
  EmptyCauldron::new()
    .mix(i2)
    .boil()
    .mix(i1)
    .pour_as::<Green>()
}


fn main() {
  Garden::<Wiggentree, 12>::new()
    .add_feeding_schedule(Monthly, magic_brownie)
    .garden()
}
