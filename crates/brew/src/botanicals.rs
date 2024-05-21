//! Botanicals grown for use in potions.
use typenum as tn;

use crate::{
  is::IsNot,
  potions::IntoRecipe,
  prelude::Occurrence,
  time::{Daily, Minutely, Schedule, Weekly, Yearly},
};

pub trait Botanical {}

pub struct Dittany;
pub struct Aconite;
pub struct Wiggentree;
pub struct Alihotsy;
pub struct Shrivelfig;
pub struct Bubotuber;

impl Botanical for Dittany {}
impl Botanical for Aconite {}
impl Botanical for Wiggentree {}
impl Botanical for Alihotsy {}
impl Botanical for Shrivelfig {}
impl Botanical for Bubotuber {}

// IsNot relationships, this is annoying because we can't specify
// this as an auto-trait with negative impl.

impl IsNot<Aconite> for Dittany {}
impl IsNot<Aconite> for Wiggentree {}
impl IsNot<Aconite> for Alihotsy {}
impl IsNot<Aconite> for Shrivelfig {}
impl IsNot<Aconite> for Bubotuber {}

impl IsNot<Dittany> for Aconite {}
impl IsNot<Dittany> for Wiggentree {}
impl IsNot<Dittany> for Alihotsy {}
impl IsNot<Dittany> for Shrivelfig {}
impl IsNot<Dittany> for Bubotuber {}

impl IsNot<Wiggentree> for Aconite {}
impl IsNot<Wiggentree> for Dittany {}
impl IsNot<Wiggentree> for Alihotsy {}
impl IsNot<Wiggentree> for Shrivelfig {}
impl IsNot<Wiggentree> for Bubotuber {}

impl IsNot<Alihotsy> for Aconite {}
impl IsNot<Alihotsy> for Dittany {}
impl IsNot<Alihotsy> for Wiggentree {}
impl IsNot<Alihotsy> for Shrivelfig {}
impl IsNot<Alihotsy> for Bubotuber {}

impl IsNot<Shrivelfig> for Aconite {}
impl IsNot<Shrivelfig> for Dittany {}
impl IsNot<Shrivelfig> for Wiggentree {}
impl IsNot<Shrivelfig> for Alihotsy {}
impl IsNot<Shrivelfig> for Bubotuber {}

impl IsNot<Bubotuber> for Aconite {}
impl IsNot<Bubotuber> for Dittany {}
impl IsNot<Bubotuber> for Wiggentree {}
impl IsNot<Bubotuber> for Alihotsy {}
impl IsNot<Bubotuber> for Shrivelfig {}

// ------------------------------

pub trait SafeDelta {
  type Min: tn::Unsigned + tn::NonZero;
  type Max: tn::Unsigned + tn::NonZero;
  type Unit: Occurrence;
}

// TODO: introduce unit conversion for different types.

impl SafeDelta for Dittany {
  type Min = tn::U1;
  type Max = tn::U3;
  type Unit = Daily;
}

impl SafeDelta for Aconite {
  type Min = tn::U2;
  type Max = tn::U2;
  type Unit = Weekly;
}

impl SafeDelta for Wiggentree {
  type Min = tn::U1;
  type Max = tn::U3;
  type Unit = Yearly;
}

impl SafeDelta for Alihotsy {
  type Min = tn::U1;
  type Max = tn::U120;
  type Unit = Minutely;
}

impl SafeDelta for Shrivelfig {
  type Min = tn::U1;
  type Max = tn::U20;
  type Unit = Daily;
}

impl SafeDelta for Bubotuber {
  type Min = tn::U258;
  type Max = tn::U2048;
  type Unit = Minutely;
}

// ------------------------------

pub struct Garden<T: Botanical, U: tn::Unsigned + tn::NonZero> {
  _plants: std::marker::PhantomData<T>,
  _count: std::marker::PhantomData<U>,
}

impl<T: Botanical, U: tn::Unsigned + tn::NonZero> Garden<T, U> {
  pub fn new(_n: tn::PInt<U>) -> Self {
    Garden {
      _plants: std::marker::PhantomData,
      _count: std::marker::PhantomData,
    }
  }

  pub fn add_schedule<S, R>(
    &mut self,
    _schedule: impl Schedule<T, S>,
    _recipe: impl IntoRecipe<R>,
  ) -> &mut Self {
    self
  }

  pub fn garden(&self) {}
}
