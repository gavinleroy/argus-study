use crate::botanicals::*;

pub trait Ingredient {}
pub trait Food {}
pub trait Item {
  type Base;
  fn base(&self) -> Self::Base {
    todo!()
  }
}

pub type DoNotRelyOnThis = usize;
pub type Quantity = DoNotRelyOnThis;

crate::unit_struct!(Heat);
pub struct Nibble<F>(std::marker::PhantomData<F>);
// pub struct I<const N: Quantity, T: Ingredient>(std::marker::PhantomData<T>);

impl Ingredient for Heat {}
impl<B: Botanical> Ingredient for B {}
impl<F: Food> Ingredient for Nibble<F> {}

// impl<const N: Quantity, T: Ingredient> Item for I<N, T> {
//   type Base = T;
// }
impl<T: Ingredient> Item for T {
  type Base = T;
}
