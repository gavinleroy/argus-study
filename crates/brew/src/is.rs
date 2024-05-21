pub trait Is<T> {}
impl<T> Is<T> for T {}

pub trait IsNot<T> {}
// impl<T> !IsNot<T> for T {}
