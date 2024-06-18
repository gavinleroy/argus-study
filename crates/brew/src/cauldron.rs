use crate::{ingredient::*, potions::*};

mod boil_dsl;
mod mix_dsl;
mod pour_dsl;

mod types {
  use super::*;
  pub type Pour<Source, Poured> = <Source as pour_dsl::PourDsl<Poured>>::Output;
  pub type Mix<Source, With> = <Source as mix_dsl::MixDsl<With>>::Output;
  pub type Boil<Source> = <Source as boil_dsl::BoilDsl>::Output;
}

mod methods {
  pub use super::{boil_dsl::*, mix_dsl::*, pour_dsl::*};
}

pub trait BrewDsl: Sized {
  fn pour_as<P>(self) -> types::Pour<Self, P>
  where
    Self: methods::PourDsl<P>,
    P: Potion,
  {
    methods::PourDsl::pour(self)
  }

  fn mix<Rhs>(self, rhs: Rhs) -> types::Mix<Self, Rhs::Base>
  where
    Rhs: Item,
    Rhs::Base: Ingredient,
    Self: methods::MixDsl<Rhs::Base>,
  {
    methods::MixDsl::mix_with(self, rhs.base())
  }

  fn boil(self) -> types::Boil<Self>
  where
    Self: methods::BoilDsl,
  {
    methods::BoilDsl::boil(self)
  }
}

// ----------------------
// Impl things

pub trait NonEmpty {}
impl<T1> NonEmpty for (T1,) {}
impl<T1, T2> NonEmpty for (T1, T2) {}
impl<T1, T2, T3> NonEmpty for (T1, T2, T3) {}
impl<T1, T2, T3, T4> NonEmpty for (T1, T2, T3, T4) {}

pub trait Cauldron {
  type Content;
}

impl<C: Cauldron> BrewDsl for C {}
impl<C> NonEmpty for C
where
  C: Cauldron,
  C::Content: NonEmpty,
{
}

pub struct MixingCauldron<T>(std::marker::PhantomData<T>);
impl<T> Cauldron for MixingCauldron<T> {
  type Content = T;
}

#[derive(Default)]
pub struct EmptyCauldron;

impl Cauldron for EmptyCauldron {
  type Content = ();
}

pub struct BoilingCauldron<T>(std::marker::PhantomData<T>);
impl<T> BoilingCauldron<T> {
  pub fn rest(self) -> impl Cauldron<Content = T> {
    MixingCauldron(std::marker::PhantomData)
  }
}
