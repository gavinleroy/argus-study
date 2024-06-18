use super::Cauldron;
use crate::potions::*;

pub trait PourDsl<P: Potion> {
  /// The type returned by the `.pour()` method.
  type Output;

  fn pour(self) -> Self::Output;
}

impl<C, P> PourDsl<P> for C
where
  C: Cauldron,
  P: Potion,
  C::Content: PourDsl<P>,
{
  type Output = <C::Content as PourDsl<P>>::Output;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

// ----------------
// Pouring remedies

impl<T1, R: Remedy> PourDsl<R> for (T1,) {
  type Output = R;
  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<T1, T2, T3, R: Remedy> PourDsl<R> for (T1, T2, T3) {
  type Output = R;
  fn pour(self) -> Self::Output {
    todo!()
  }
}

// ---------------
// Pouring poisons

impl<T1, T2, P: Poison> PourDsl<P> for (T1, T2) {
  type Output = P;
  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<T1, T2, T3, T4, P: Poison> PourDsl<P> for (T1, T2, T3, T4) {
  type Output = P;
  fn pour(self) -> Self::Output {
    todo!()
  }
}
