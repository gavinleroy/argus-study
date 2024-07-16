use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[botanical Reflower]
     [botanical Bubotuber]] ==> Potion
}

#[test]
fn is_correct() {
  spec(fertilize);
}
