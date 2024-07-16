use crate::botanicals::*;

pub(crate) trait PlantSafe: Ingredient {}
pub trait Ingredient {}

pub trait HumanFood {}

pub struct Liquified<F>(std::marker::PhantomData<F>);

impl<B> Ingredient for B {}

impl<B: Botanical> PlantSafe for B {}
impl<F: HumanFood> PlantSafe for Liquified<F> {}
