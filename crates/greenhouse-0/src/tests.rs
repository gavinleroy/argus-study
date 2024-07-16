use brew::describe_recipe;

use super::*;

describe_recipe! {
  spaghetti_spec
    [[food Tomato]
     [food Meatball]
     [food Pasta]] ==> Remedy
}

#[test]
fn is_correct() {
  spaghetti_spec(make_spaghetti);
}
