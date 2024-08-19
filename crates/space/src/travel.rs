use std::marker::PhantomData;

use crate::{
  dir::{Direction, Down, Left, Right, Up},
  num::{self, Num},
  pos::{Origin, Pos},
};

mod blast_dsl;
mod charge_dsl;
mod collect_dsl;
mod move_dsl;

mod types {
  use super::*;
  pub type Move<Rocket, D> = <Rocket as move_dsl::MoveDsl<D>>::Output;
  pub type Charge<Rocket, C> = <Rocket as charge_dsl::ChargeDsl<C>>::Output;
  pub type Blast<Rocket, C> = <Rocket as blast_dsl::BlastDsl<C>>::Output;
}

mod methods {
  pub use super::{
    blast_dsl::*, charge_dsl::*, collect_dsl::CollectDsl as ProbeDsl,
    move_dsl::*,
  };
}

pub trait Rocket {
  type Location: Pos;
  type Charge: Num;
  type Fuel: Num;
  type Dir: Direction;
}

// You can only collect samples if it has `location == destination`

pub trait ExplorationDsl: Sized {
  fn left(self) -> types::Move<Self, Left>
  where
    Self: methods::MoveDsl<Left>,
  {
    methods::MoveDsl::r#move(self, Left)
  }

  fn right(self) -> types::Move<Self, Right>
  where
    Self: methods::MoveDsl<Right>,
  {
    methods::MoveDsl::r#move(self, Right)
  }

  fn up(self) -> types::Move<Self, Up>
  where
    Self: methods::MoveDsl<Up>,
  {
    methods::MoveDsl::r#move(self, Up)
  }

  fn down(self) -> types::Move<Self, Down>
  where
    Self: methods::MoveDsl<Down>,
  {
    methods::MoveDsl::r#move(self, Down)
  }

  fn forward<X>(self, x: X) -> types::Blast<Self, X>
  where
    Self: methods::BlastDsl<X>,
  {
    methods::BlastDsl::blast(self, x)
  }

  fn charge_engines<X>(self, x: X) -> types::Charge<Self, X>
  where
    Self: methods::ChargeDsl<X>,
  {
    methods::ChargeDsl::charge(self, x)
  }

  fn probe<P, C, Marker>(self, config: C)
  where
    Self: methods::ProbeDsl<P, C, Marker>,
  {
    methods::ProbeDsl::collect(self, config);
  }
}

impl<R: Rocket> ExplorationDsl for R {}

pub struct IntergalacticRocket<Loc, Ch, F, Dir>(PhantomData<(Loc, Ch, F, Dir)>);

pub fn make_rocket() -> IntergalacticRocket<Origin, num::Zero, num::Ten, Up> {
  IntergalacticRocket(PhantomData)
}

impl<L: Pos, C: Num, F: Num, Dir: Direction> Rocket
  for IntergalacticRocket<L, C, F, Dir>
{
  type Location = L;
  type Charge = C;
  type Fuel = F;
  type Dir = Dir;
}
