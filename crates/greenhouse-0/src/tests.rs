use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[mineral Nitrogen]
     [mineral Phosphorus]
     [botanical Alihotsy]] ==> Remedy
}

#[test]
fn is_correct() {
  spec(recipe);
}
