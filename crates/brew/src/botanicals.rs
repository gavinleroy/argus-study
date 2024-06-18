//! Botanicals grown for use in potions.
use crate::{
  is::Neq,
  potions::{IntoRecipe, Poison},
  time::AsSchedule,
};

pub trait Botanical {}

crate::unit_struct! {
  Dittany,
  Aconite,
  Wiggentree,
  Alihotsy,
  Shrivelfig,
  Bubotuber
}

macro_rules! botanicals {
  ($($name:ident),*) => {
    $(
      impl Botanical for $name {}
    )*
  };
}

botanicals! {
  Dittany,
  Aconite,
  Wiggentree,
  Alihotsy,
  Shrivelfig,
  Bubotuber
}

// Neq relationships, this is annoying because we can't specify
// this as an auto-trait with negative impl.

impl Neq<Aconite> for Dittany {}
impl Neq<Aconite> for Wiggentree {}
impl Neq<Aconite> for Alihotsy {}
impl Neq<Aconite> for Shrivelfig {}
impl Neq<Aconite> for Bubotuber {}

impl Neq<Dittany> for Aconite {}
impl Neq<Dittany> for Wiggentree {}
impl Neq<Dittany> for Alihotsy {}
impl Neq<Dittany> for Shrivelfig {}
impl Neq<Dittany> for Bubotuber {}

impl Neq<Wiggentree> for Aconite {}
impl Neq<Wiggentree> for Dittany {}
impl Neq<Wiggentree> for Alihotsy {}
impl Neq<Wiggentree> for Shrivelfig {}
impl Neq<Wiggentree> for Bubotuber {}

impl Neq<Alihotsy> for Aconite {}
impl Neq<Alihotsy> for Dittany {}
impl Neq<Alihotsy> for Wiggentree {}
impl Neq<Alihotsy> for Shrivelfig {}
impl Neq<Alihotsy> for Bubotuber {}

impl Neq<Shrivelfig> for Aconite {}
impl Neq<Shrivelfig> for Dittany {}
impl Neq<Shrivelfig> for Wiggentree {}
impl Neq<Shrivelfig> for Alihotsy {}
impl Neq<Shrivelfig> for Bubotuber {}

impl Neq<Bubotuber> for Aconite {}
impl Neq<Bubotuber> for Dittany {}
impl Neq<Bubotuber> for Wiggentree {}
impl Neq<Bubotuber> for Alihotsy {}
impl Neq<Bubotuber> for Shrivelfig {}

// ------------------------------

pub trait AsGarden {}
impl<T: Botanical, const N: usize> AsGarden for Garden<T, N> {}

pub struct Garden<T: Botanical, const N: usize> {
  _plants: std::marker::PhantomData<[T; N]>,
}

impl<T: Botanical, const N: usize> Garden<T, N> {
  pub fn new() -> Self {
    Garden {
      _plants: std::marker::PhantomData,
    }
  }

  pub fn add_feeding_schedule<R>(
    &mut self,
    _schedule: impl AsSchedule,
    _recipe: impl IntoRecipe<R>,
  ) -> &mut Self {
    self
  }

  pub fn sabotage<P, S, R>(&mut self, _recipe: R) -> &mut Self
  where
    P: Poison,
    R: IntoRecipe<S, Output = P>,
  {
    self
  }

  pub fn garden(&self) {}
}

pub trait ParallelFeed {
  fn feed_in_parallel<S, R>(&self, _recipe: R)
  where
    R: IntoRecipe<S> + Send + Sync,
    R::Output: Send + Sync;
}

impl ParallelFeed for Vec<&dyn AsGarden> {
  fn feed_in_parallel<S, R>(&self, _recipe: R)
  where
    R: IntoRecipe<S> + Send + Sync,
    R::Output: Send + Sync,
  {
    todo!()
  }
}
