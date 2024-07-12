mod pretty;
mod database;
mod name;

use database::Database;
use name::Name;

fn main() {
  let mut actors = Database::new();

  actors.insert(Name::new("Molly", "Ringwald"));
  actors.insert(Name::new("Jon", "Cryer"));
  actors.insert(Name::new("Andrew", "McCarthy"));

  pretty::in_pink(&actors);
}