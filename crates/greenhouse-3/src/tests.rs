use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[botanical Reflower]
     [mineral Nitrogen]] ==> Potion
}

#[test]
fn is_correct() {
  spec(supplement);
}
