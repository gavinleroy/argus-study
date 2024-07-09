use super::{Cauldron, MixingCauldron};
use crate::{ingredient::Ingredient, count::Add1};

pub trait MixDsl<Rhs> {
  type Output;

  fn mix_with(self, rhs: Rhs) -> Self::Output;
}

impl<C, Rhs> MixDsl<Rhs> for C
where
  C: Cauldron,
  C::IngredientCount: Add1,
  Rhs: Ingredient,
{
  type Output = MixingCauldron<<C::IngredientCount as Add1>::Output, C::Temperature>;
  fn mix_with(self, _rhs: Rhs) -> Self::Output {
    todo!()
  }
}
