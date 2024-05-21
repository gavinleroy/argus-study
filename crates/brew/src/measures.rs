pub trait Quantity {}

pub struct Dash;
pub struct Pinch;
pub struct Scoop;
pub struct Heaped<T: Quantity>(std::marker::PhantomData<T>);

impl Quantity for Dash {}
impl Quantity for Pinch {}
impl Quantity for Scoop {}
impl<T: Quantity> Quantity for Heaped<T> {}
