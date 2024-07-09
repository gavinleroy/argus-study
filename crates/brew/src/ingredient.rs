use crate::botanicals::*;

pub trait Ingredient {}

pub trait HumanFood {}

pub trait Item {
  type Base;
  fn base(&self) -> Self::Base {
    todo!()
  }
}

pub struct Liquified<F>(std::marker::PhantomData<F>);

impl<B: Botanical> Ingredient for B {}
impl<F: HumanFood> Ingredient for Liquified<F> {}

impl<T: Ingredient> Item for T {
  type Base = T;
}
