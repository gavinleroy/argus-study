use space::prelude::*;

fn collect_debris(query: Query<(Screw, Bolt, UFO)>) {
  for (s, b, ufo) in &query {
    println!("Collecting debris: (screw, bolt, ufo) ({s:?}, {b:?}, {ufo:?})");
  }
}

fn main() {
  make_rocket()
    .up()
    .charge_engines(One)
    .forward(One)
    .right()
    .charge_engines(Two)
    .forward(Two)
    .probe(collect_debris)
}
