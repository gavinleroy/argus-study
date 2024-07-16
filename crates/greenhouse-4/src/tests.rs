use super::*;

#[test]
fn is_correct() {
  fn is_recipe<R>(_: impl IntoRecipe<R>) {}
  is_recipe(speed_feed);
}
