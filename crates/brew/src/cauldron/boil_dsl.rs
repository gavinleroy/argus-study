use super::{BoilingCauldron, Cauldron, NonEmpty};
use crate::{add::Addable, ingredient::Heat};

pub trait BoilDsl {
  type Output;

  fn boil(self) -> Self::Output;
}

impl<C> BoilDsl for C
where
  C: Cauldron + NonEmpty,
  C::Content: Addable<Heat>,
{
  type Output = BoilingCauldron<<C::Content as Addable<Heat>>::Output>;

  fn boil(self) -> Self::Output {
    todo!()
  }
}
