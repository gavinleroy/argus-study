mod botanicals;
mod cauldron;
mod count;
mod ingredient;
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

#[macro_export]
macro_rules! make_simple {
    ($vis:vis $c:ident ==> $($t:tt),*) => {
        $crate::unit_struct!($($t),*);
        $vis trait $c {}
        $(
            impl $c for $t {}
        )*
    }
}
