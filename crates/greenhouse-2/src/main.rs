#[cfg(test)]
mod tests;

use brew::prelude::*;

async fn shrivel(i1: Shrivelfig) -> impl Potion {
  // NOTE: I definitely want to return a `Pink` potion here.
  EmptyCauldron::new()
    .mix(i1)
    .boil()
    .rest()
    .pour_as::<Pink, _>()
}

fn help_out_the_neighbors(the_neighbors_figs: &mut Garden<Shrivelfig, 6>) {
  // FIXME: I love my neighbors, but their figs are out of control. I'm
  // garden sitting this weekend and want to sabotage feeding them.
  //
  // However, my provided recipe doesn't type check and I don't know why.
  the_neighbors_figs.sabotage(shrivel);
}

fn main() {
  help_out_the_neighbors(&mut Garden::new());
}
