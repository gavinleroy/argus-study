//! Potion brewing system.

// use std::{future::Future, ops::Add};
// use typenum as tn;

use std::future::Future;

use crate::{
    is::Neq,
    ingredient::Item,
};

// --------------
// Potions

pub trait Potion {}
pub trait Poison: Potion {}
pub trait Remedy: Potion {}
pub trait IntoPotion {
    type Output: Potion;
}

crate::unit_struct! {
    Green,
    Blue,
    Pink,
    Yellow
}

crate::impl_as! {
    Potion ==>
    Green,
    Blue,
    Pink,
    Yellow
}

crate::impl_as! {
    Poison ==>
    Pink,
    Yellow
}

crate::impl_as! {
    Remedy ==>
    Green,
    Blue
}

// --------------
// Recipes

pub trait IntoRecipe<T> {
  type Output: Potion;
}

impl<F, T1, Out, Res> IntoRecipe<(T1, Out, Res)> for F
where
  F: FnOnce(T1) -> Out + Send,
  T1: Item,
  Out: Future<Output = Res> + Send,
  Res: Potion,
{
  type Output = Res;
}

impl<F, T1, T2, Out, Res> IntoRecipe<(T1, T2, Out, Res)> for F
where
  F: FnOnce(T1, T2) -> Out + Send,
  T1: Item,
  T2: Item,
  T2::Base: Neq<T1::Base>,
  Out: Future<Output = Res> + Send,
  Res: Potion,
{
  type Output = Res;
}

impl<F, T1, T2, T3, Out, Res> IntoRecipe<(T1, T2, T3, Out, Res)> for F
where
  F: FnOnce(T1, T2, T3) -> Out + Send,
  T1: Item,
  T2: Item,
  T3: Item,
  T2::Base: Neq<T1::Base>,
  T3::Base: Neq<T1::Base>,
  T3::Base: Neq<T2::Base>,
  Out: Future<Output = Res> + Send,
  Res: Potion,
{
  type Output = Res;
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::botanicals::*;

  fn is_recipe<R: IntoRecipe<T>, T>(_: R) {}

  #[test]
  fn test_1() {
      async fn f(
          _: Dittany,
      ) -> Blue { todo!() }
      is_recipe(f);
  }

  #[test]
  fn test_2() {
      async fn f(
          _: Dittany,
          _: Aconite,
          _: Wiggentree,
      ) -> Blue { todo!() }
      is_recipe(f);
  }

  #[test]
  fn test_3() {
      async fn f(
          _: Dittany,
          _: Wiggentree,
      ) -> Pink { todo!() }
      is_recipe(f);
  }

  // ///```compile_fail
  // /// async fn f(_: I<1, Dash, usize>) {}
  // /// is_recipe(f);
  // ///```
  // #[test]
  // fn test_2() {}
}
