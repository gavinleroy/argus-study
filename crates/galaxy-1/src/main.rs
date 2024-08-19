use space::prelude::*;

fn collect_meteoroid(query: Query<Meteoroid>) {
  for m in &query {
    println!("Meteoroid collected! {m:?}");
  }
}

fn main() {
  make_rocket()
    // Move upward one
    .up()
    .charge_engines(One)
    .forward(One)
    // Move right one
    .right()
    .charge_engines(One)
    .forward(One)
    // Collect
    .probe(collect_meteoroid)
}
