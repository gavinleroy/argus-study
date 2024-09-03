use space::prelude::*;

// NOTE: This logic is sufficient to collect the meteoroid, 
// please don't change the function signature or body.
fn collect_meteoroid(query: Finder<Meteoroid>) {
  for m in &query {
    println!("Meteoroid collected! {m:?}");
  }
}

// TASK: There is a meteoroid within two AU^2 of position (1, 1) that needs to be collected. 
// If we can get to a suitable position of the description it should be within collection range.
fn main() {
  // Start the mission from the space center origin
  Rocket::from_origin()
    .up()
    .forward(One)
    .right()
    .forward(One)
    .probe(collect_meteoroid)
}
