use super::{Cauldron, Cold, Hot, MixingCauldron, Temperature, Warm};

pub trait RestDsl {
  type Output;

  fn rest(self) -> Self::Output;
}

impl<C> RestDsl for C
where
  C: Cauldron,
  C::Temperature: RestDsl,
  <C::Temperature as RestDsl>::Output: Temperature,
{
  type Output =
    MixingCauldron<C::IngredientCount, <C::Temperature as RestDsl>::Output>;

  fn rest(self) -> Self::Output {
    todo!()
  }
}

impl RestDsl for Hot {
  type Output = Warm;

  fn rest(self) -> Self::Output {
    todo!()
  }
}

impl RestDsl for Warm {
  type Output = Cold;

  fn rest(self) -> Self::Output {
    todo!()
  }
}

impl RestDsl for Cold {
  type Output = Cold;

  fn rest(self) -> Self::Output {
    todo!()
  }
}
