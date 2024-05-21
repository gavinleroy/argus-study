//! Potion brewing system.

use std::ops::Add;
use std::future::Future;
use typenum as tn;

use crate::botanicals::*;
use crate::measures::*;
use crate::is::{Is, IsNot};

pub trait Potion {}

pub trait IntoRecipe<T> {}

pub trait Ingredient<N: tn::Integer> {
    type Q: Quantity;
    type E: Botanical;
}

pub struct I<N: tn::Integer, Q, E>(N, Q, E);
impl<N, Q, E> Ingredient<N> for I<N, Q, E>
    where
    N: tn::Integer,
    Q: Quantity,
    E: Botanical
{
    type Q = Q;
    type E = E;
}

impl<
    I1, N1, Q1, E1,
    I2, N2, Q2, E2,
    I3, N3, Q3, E3,
    Res, Out, F> IntoRecipe<(I1,I2,I3,N1,N2,N3,Q1,Q2,Q3,E1,E2,E3)> for F
where
    N1: Add<N2> + tn::Unsigned + tn::NonZero,
    <N1 as Add<N2>>::Output: tn::Unsigned + tn::NonZero,
    N2: tn::Unsigned + tn::NonZero,
    N3: Is<tn::Sum<tn::PInt<N1>, tn::PInt<N2>>> + tn::Integer,
    Q1: Quantity,
    Q2: Quantity,
    Q3: Quantity,
    E1: Botanical,
    E2: Botanical + IsNot<E1>,
    E3: Botanical + IsNot<E1> + IsNot<E2>,
    I1: Ingredient<tn::PInt<N1>, Q = Q1, E = E1>,
    I2: Ingredient<tn::PInt<N2>, Q = Q2, E = E2>,
    I3: Ingredient<N3, Q = Q3, E = E3>,
    F: Fn(I1, I2, I3) -> Out,
    Out: Future<Output = Res>,
{
}

impl<
    I1, N1, Q1, E1,
    I2, N2, Q2, E2,
    I3, N3, Q3, E3,
    I4, N4, Q4, E4,
    I5, N5, Q5, E5,
    Res, Out, F> IntoRecipe<(I1,I2,I3,I4,I5,N1,N2,N3,N4,N5,Q1,Q2,Q3,Q4,Q5,E1,E2,E3,E4,E5)> for F
where
    N1: Add<N2> + tn::Unsigned + tn::NonZero,
    <N1 as Add<N2>>::Output: tn::Unsigned + tn::NonZero,
    N2: tn::Unsigned + tn::NonZero,
    N3: Is<tn::Sum<tn::PInt<N1>, tn::PInt<N2>>> + tn::Integer,
    N4: Is<N2> + tn::Unsigned + tn::NonZero,
    N5: Is<N1> + tn::Unsigned + tn::NonZero,
    Q1: Quantity,
    Q2: Quantity,
    Q3: Quantity,
    Q4: Quantity,
    Q5: Quantity,
    E1: Botanical,
    E2: Botanical + IsNot<E1>,
    E3: Botanical + IsNot<E1> + IsNot<E2>,
    E4: Botanical + IsNot<E1> + IsNot<E2> + IsNot<E3>,
    E5: Botanical + IsNot<E1> + IsNot<E2> + IsNot<E3> + IsNot<E4>,
    I1: Ingredient<tn::PInt<N1>, Q = Q1, E = E1>,
    I2: Ingredient<tn::PInt<N2>, Q = Q2, E = E2>,
    I3: Ingredient<N3, Q = Q3, E = E3>,
    I4: Ingredient<tn::PInt<N4>, Q = Q4, E = E4>,
    I5: Ingredient<tn::PInt<N5>, Q = Q5, E = E5>,
    F: Fn(I1, I2, I3, I4, I5) -> Out,
    Out: Future<Output = Res>,
{
}

#[cfg(test)]
mod test {
    use super::*;

    fn is_recipe<R: IntoRecipe<T>, T>(_: R) {}

    #[test]
    fn test_1() {
        async fn f(_: I<tn::P1, Dash, Dittany>, _: I<tn::P1, Dash, Aconite>, _: I<tn::P2, Dash, Wiggentree>) {}
        is_recipe(f);
    }

    ///```compile_fail
    /// async fn f(_: I<1, Dash, usize>) {}
    /// is_recipe(f);
    ///```
    #[test]
    fn test_2() {}
}
