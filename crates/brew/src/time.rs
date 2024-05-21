use crate::botanicals::{Botanical, SafeDelta};
use crate::is::Is;
use typenum as tn;

pub trait Occurrence: 'static {}

pub struct Delta<T: Occurrence, N: tn::Integer> {
    _phantom: std::marker::PhantomData<(T, N)>,
}

impl<T, N> Delta<T, N>
where
    T: Occurrence,
    N: tn::Integer,
{
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait Schedule<For: Botanical, T>: 'static {}

pub struct Minutely;
pub struct Hourly;
pub struct Daily;
pub struct Weekly;
pub struct Monthly;
pub struct Yearly;

impl Occurrence for Minutely {}
impl Occurrence for Hourly {}
impl Occurrence for Daily {}
impl Occurrence for Weekly {}
impl Occurrence for Monthly {}
impl Occurrence for Yearly {}

impl<B, N, T, U, Mn, Mx> Schedule<B, (U, Mn, Mx)> for Delta<T, N>
where
    Mn: tn::Unsigned + tn::NonZero,
    U: tn::Unsigned + tn::NonZero,
    Mx: tn::Unsigned + tn::NonZero,

    tn::PInt<Mn>: tn::Cmp<tn::PInt<U>>,
    tn::PInt<Mn>: tn::private::IsLessOrEqualPrivate<
        tn::PInt<U>,
        <tn::PInt<Mn> as tn::Cmp<tn::PInt<U>>>::Output,
    >,
    tn::PInt<U>: tn::Cmp<tn::PInt<Mx>>,
    tn::PInt<U>: tn::private::IsLessOrEqualPrivate<
        tn::PInt<Mx>,
        <tn::PInt<U> as tn::Cmp<tn::PInt<Mx>>>::Output,
    >,
    T: Occurrence,
    B: Botanical + SafeDelta<Min = Mn, Max = Mx, Unit = T>,
    N: Is<tn::PInt<U>> + tn::Integer,
    tn::LeEq<tn::PInt<Mn>, tn::PInt<U>>: Is<tn::True>,
    tn::LeEq<tn::PInt<U>, tn::PInt<Mx>>: Is<tn::True>,
{
}
