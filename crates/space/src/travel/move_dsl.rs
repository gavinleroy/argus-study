use super::*;
use crate::{dir::Direction, num::IsZero};

pub trait MoveDsl<D> {
  type Output;

  fn r#move(self, d: D) -> Self::Output;
}

impl<R, D> MoveDsl<D> for R
where
  R: Rocket,
  D: Direction,
  // You can only move a rocket if it has charge `y == 0`
  R::Charge: IsZero,
{
  type Output = IntergalacticRocket<R::Location, R::Charge, R::Fuel, D>;

  fn r#move(self, d: D) -> Self::Output {
    todo!()
  }
}
