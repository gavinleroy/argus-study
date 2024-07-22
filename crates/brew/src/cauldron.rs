use std::marker::PhantomData;

use crate::{count::*, ingredient::*, potions::*};

mod boil_dsl;
mod mix_dsl;
mod pour_dsl;
mod rest_dsl;

mod types {
  use super::*;
  pub type Pour<Source, Poured> = <Source as pour_dsl::PourDsl<Poured>>::Output;
  pub type Mix<Source, With> = <Source as mix_dsl::MixDsl<With>>::Output;
  pub type Boil<Source> = <Source as boil_dsl::BoilDsl>::Output;
  pub type Rest<Source> = <Source as rest_dsl::RestDsl>::Output;
}

mod methods {
  pub use super::{boil_dsl::*, mix_dsl::*, pour_dsl::*, rest_dsl::*};
}

crate::make_simple! {
  pub Temperature ==>
  Cold,
  Warm,
  Hot
}

trait CoolEnough {}
crate::impl_as!(CoolEnough ==> Cold, Warm);

pub trait BrewDsl: Sized {
  fn pour_as<P>(self) -> types::Pour<Self, P>
  where
    Self: methods::PourDsl<P>,
    P: Potion,
  {
    methods::PourDsl::pour(self)
  }

  fn mix<Rhs>(self, rhs: Rhs) -> types::Mix<Self, Rhs>
  where
    Rhs: Ingredient,
    Self: methods::MixDsl<Rhs>,
  {
    methods::MixDsl::mix_with(self, rhs)
  }

  fn boil(self) -> types::Boil<Self>
  where
    Self: methods::BoilDsl,
  {
    methods::BoilDsl::boil(self)
  }

  fn rest(self) -> types::Rest<Self>
  where
    Self: methods::RestDsl,
  {
    methods::RestDsl::rest(self)
  }
}

// ----------------------
// Impl things

pub trait NonEmpty {}
crate::impl_as! {
  NonEmpty ==>
  One,
  Two,
  Three,
  Four,
  Five,
  Six
}

pub trait Cauldron {
  type IngredientCount: Count;
  type Temperature: Temperature;
}

impl<C: Cauldron> BrewDsl for C {}
impl<C, T> Cauldron for MixingCauldron<C, T>
where
  C: Count,
  T: Temperature,
{
  type IngredientCount = C;
  type Temperature = T;
}

impl<C> NonEmpty for C
where
  C: Cauldron,
  C::IngredientCount: NonEmpty,
{
}

pub struct MixingCauldron<C: Count, T: Temperature>(
  std::marker::PhantomData<(C, T)>,
);

impl<C: Count, T: Temperature> MixingCauldron<C, T> {
  pub(self) fn new() -> MixingCauldron<C, T> {
    MixingCauldron(PhantomData)
  }
}

pub struct EmptyCauldron;

impl EmptyCauldron {
  pub fn new() -> MixingCauldron<Zero, Cold> {
    MixingCauldron(PhantomData)
  }
}
