use std::ops::{Add, Sub};

use super::*;
use crate::{dir::Direction, num::LessOrEqual, pos::PosUpdate};

pub trait BlastDsl<X> {
  type Output;

  fn blast(self, x: X) -> Self::Output;
}

impl<R, X> BlastDsl<X> for R
where
  R: Rocket,
  X: Num,
  // You can only move a rocket `x` if it has a charge `y >= x`
  X: LessOrEqual<R::Charge>,
  R::Location: PosUpdate<R::Dir, X>,
  R::Charge: Sub<X>,
{
  type Output = IntergalacticRocket<
    <R::Location as PosUpdate<R::Dir, X>>::Output,
    <R::Charge as Sub<X>>::Output,
    R::Fuel,
    R::Dir,
  >;

  fn blast(self, x: X) -> Self::Output {
    todo!()
  }
}
