use space::prelude::*;

fn collect_debris(query: Query<(Bolt, AlienCrap)>) {
  for (b, a) in &query {
    println!("Bolt & AlienCrap collected! {b:?} {a:?}");
  }
}

fn main() {
  make_rocket()
    .up()
    .charge_engines(Five)
    .forward(Three)
    .right()
    .forward(Two)
    .probe(collect_debris)
}
