use space::prelude::*;

#[derive(Collectible, Debug)]
struct FermiBubble;

fn collect(bubble: FermiBubble, query: Finder<(Meteoroid, Asteroid)>) {
  println!("Collected Fermi Bubble! {bubble:?}");

  for (m, a) in &query {
    println!("Meteoroid & Asteroid collected! {m:?} {a:?}");
  }
}

// TASK: there's a fermi bubble at (1, 0), such a rarity; let's collect it!
// The bubble contains meteoroids and asteroids, let's collect them too. The 
// flight plan below is correct and shouldn't need modification.
fn main() {
  Rocket::from_origin()
    .right()
    .forward(One)
    .probe(collect)
}
