pub trait Addable<Rhs> {
  type Output;
}

impl<T> Addable<T> for () {
  type Output = (T,);
}

impl<E1, T> Addable<T> for (E1,) {
  type Output = (E1, T);
}

impl<E1, E2, T> Addable<T> for (E1, E2) {
  type Output = (E1, E2, T);
}

impl<E1, E2, E3, T> Addable<T> for (E1, E2, E3) {
  type Output = (E1, E2, E3, T);
}
