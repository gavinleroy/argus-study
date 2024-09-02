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

// TASK: Create a garden of wiggentrees and feed them daily with the following 
// fertilizer mixture recipe: 1 part nitrogen, 1 part phosphorus, and 1 part alihotsy.
// The resulting potion should be *blue*. 
// The below setup code should not need modification.
fn main() {
  Garden::<Wiggentree, 1>::new()
    .add_feeding_schedule(Daily, recipe)
    .garden()
}
