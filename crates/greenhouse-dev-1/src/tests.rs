use super::*;
use brew::describe_recipe;

describe_recipe! {
  dirty_cake_spec
    [[botanical Shrivelfig]
     [botanical Wiggentree]] ==> Poison
}

#[test]
fn is_correct() {
  dirty_cake_spec(dirty_cake);
}
