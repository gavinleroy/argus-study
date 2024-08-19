use std::ops::{Add, Sub};

use super::*;
use crate::{dir::Direction, num::LessOrEqual};

pub trait ChargeDsl<X> {
  type Output;

  fn charge(self, x: X) -> Self::Output;
}

impl<R, X> ChargeDsl<X> for R
where
  R: Rocket,
  X: Num,
  // You can only charge a rocket `x` if it has fuel `y >= x`
  X: LessOrEqual<R::Fuel>,
  R::Charge: Add<X>,
  R::Fuel: Sub<X>,
{
  type Output = IntergalacticRocket<
    R::Location,
    <R::Charge as Add<X>>::Output,
    <R::Fuel as Sub<X>>::Output,
    R::Dir,
  >;

  fn charge(self, x: X) -> Self::Output {
    todo!()
  }
}
