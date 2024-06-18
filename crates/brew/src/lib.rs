mod add;
mod botanicals;
mod cauldron;
mod ingredient;
mod is;
mod potions;
mod time;

pub mod prelude {
  pub use crate::{
    botanicals::*, cauldron::*, ingredient::*, potions::*, time::*,
  };
}

#[macro_export]
macro_rules! unit_struct {
    ($($t:ident),*) => {
        $(
            pub struct $t(());
        )*
    }
}

#[macro_export]
macro_rules! impl_as {
    ($c:ident ==> $($t:tt),*) => {
        $(
            impl $c for $t {}
        )*
    }
}
