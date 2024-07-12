use super::*;
use brew::describe_recipe;

describe_recipe! {
  gloop_spec
    [[botanical Reflower]
     [food Mozerella]
     [botanical Alihotsy]
     [botanical Shrivelfig]] ==> Remedy
}

#[test]
fn is_correct() {
  gloop_spec(gloop);
}
