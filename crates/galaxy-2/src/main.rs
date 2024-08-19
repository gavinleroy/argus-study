use space::prelude::*;

fn probe_with_ufo_check<Marker>(
  r: impl Rocket,
  config: impl IntoProbe<(), (), Marker>,
) {
  r.probe((
    config,
    |depot: &mut CollectionDeposit, ufo: Query<Option<UFO>>| {
      for ufo in &ufo {
        if let Some(ufo) = ufo {
          println!("UFO sighting! {ufo:?}");
          depot.add_debris(ufo);
        }
      }
    },
  ))
}

fn collect_debris(query: Query<(Bolt, Screw)>) {
  for (bolt, screw) in &query {
    println!("Collecting: (bolt, screw) ({:?}, {:?})", bolt, screw);
  }
}

fn main() {
  let r = make_rocket()
    .right()
    .charge_engines(Seven)
    .forward(Seven);

  probe_with_ufo_check(r, collect_debris)
}
