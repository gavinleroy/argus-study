use super::{Cauldron, Cold, NonEmpty, Warm};
use crate::{count::*, potions::*};

pub struct IsColdRemedy;
pub struct IsWarmRemedy;
pub struct IsColdPoison;
pub struct IsWarmPoison;

pub trait PourDsl<P: Potion, Marker> {
  /// The type returned by the `.pour()` method.
  type Output;

  fn pour(self) -> Self::Output;
}

impl<C, R, Count> PourDsl<R, IsColdRemedy> for C
where
  C: Cauldron<Temperature = Cold, IngredientCount = Count>,
  R: Remedy,
  Count: NonEmpty + IsOdd,
{
  type Output = R;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<C, R, Count> PourDsl<R, IsWarmRemedy> for C
where
  C: Cauldron<Temperature = Warm, IngredientCount = Count>,
  R: Remedy,
  Count: NonEmpty + IsEven,
{
  type Output = R;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<C, P, Count> PourDsl<P, IsColdPoison> for C
where
  C: Cauldron<Temperature = Cold, IngredientCount = Count>,
  P: Poison,
  Count: NonEmpty + IsEven,
{
  type Output = P;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<C, P, Count> PourDsl<P, IsWarmPoison> for C
where
  C: Cauldron<Temperature = Warm, IngredientCount = Count>,
  P: Poison,
  Count: NonEmpty + IsOdd,
{
  type Output = P;

  fn pour(self) -> Self::Output {
    todo!()
  }
}
