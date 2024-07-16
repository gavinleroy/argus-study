use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[botanical Dittany]
     [botanical Bubotuber]] ==> Potion
}

#[test]
fn is_correct() {
  spec(magic_brownie);
}
