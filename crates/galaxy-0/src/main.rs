use space::prelude::*;

#[derive(Resource, Debug)]
struct FermiBubble;

fn collect(bubble: FermiBubble, query: Query<(Meteoroid, Asteroid)>) {
  println!("Collected Fermi Bubble! {bubble:?}");

  for (m, a) in &query {
    println!("Meteoroid & Asteroid collected! {m:?} {a:?}");
  }
}

fn main() {
  Rocket::from_origin()
    .right()
    .forward(One)
    .probe(collect)
}
