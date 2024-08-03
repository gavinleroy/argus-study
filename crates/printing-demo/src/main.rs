mod pretty;
mod database;
mod name;

use database::Database;
use name::Name;

fn print_to_log<T, U>(vs: U) 
where
  U: AsRef<[T]>,
  T: std::fmt::Debug,
{
  todo!()
}

fn main() {
  let mut actors = Database::new();

  actors.insert(Name::new("Molly", "Ringwald"));
  actors.insert(Name::new("Jon", "Cryer"));
  actors.insert(Name::new("Andrew", "McCarthy"));

  print_to_log(actors.buffer());

  pretty::in_pink(&actors);
}