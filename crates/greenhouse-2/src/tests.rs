use brew::describe_recipe;

use super::*;

describe_recipe! {
  spec
    [[botanical Shrivelfig]] ==> Poison
}

#[test]
fn is_correct() {
  spec(shrivel);
}
