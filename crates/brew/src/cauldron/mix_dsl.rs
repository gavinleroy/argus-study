use super::{Cauldron, MixingCauldron};
use crate::{add::Addable, ingredient::Ingredient};

pub trait MixDsl<Rhs> {
  type Output;

  fn mix_with(self, rhs: Rhs) -> Self::Output;
}

impl<C, Rhs> MixDsl<Rhs> for C
where
  C: Cauldron,
  Rhs: Ingredient,
  C::Content: Addable<Rhs>,
{
  type Output = MixingCauldron<<C::Content as Addable<Rhs>>::Output>;
  fn mix_with(self, _rhs: Rhs) -> Self::Output {
    todo!()
  }
}
