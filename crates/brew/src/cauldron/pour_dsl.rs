use super::{Cauldron, Cold, CoolEnough, NonEmpty, Warm};
use crate::{count::*, potions::*};

pub trait PourDsl<P: Potion> {
  /// The type returned by the `.pour()` method.
  type Output;

  fn pour(self) -> Self::Output;
}

impl<C, P> PourDsl<P> for C
where
  C: Cauldron,
  P: Potion,
  C::IngredientCount: NonEmpty,
  C::Temperature: CoolEnough,
  (C::IngredientCount, C::Temperature): PourDsl<P>,
{
  type Output = <(C::IngredientCount, C::Temperature) as PourDsl<P>>::Output;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

macro_rules! impl_remedy {
  (@ $tmp:ident : $($c:ty),*) => {
    $(
      impl<R> PourDsl<R> for ($c, $tmp)
        where
        R: Remedy,
      {
        type Output = R;
        fn pour(self) -> Self::Output {
          todo!()
        }
      }
    )*
  }
}

macro_rules! impl_poison {
  (@ $tmp:ident : $($c:ty),*) => {
    $(
      impl<P> PourDsl<P> for ($c, $tmp)
        where
        P: Poison,
      {
        type Output = P;
        fn pour(self) -> Self::Output {
          todo!()
        }
      }
    )*
  }
}

impl_remedy! {
  @ Cold :
  One, Three, Five
}

impl_remedy! {
  @ Warm :
  Two, Four, Six
}

impl_poison! {
  @ Cold :
  Two, Four, Six
}

impl_poison! {
  @ Warm :
  One, Three, Five
}
