#[cfg(test)]
mod tests;

use brew::prelude::*;

async fn shrivel(i1: Shrivelfig) -> impl Potion {
  EmptyCauldron::new().mix(i1).boil().rest().pour_as::<Pink>()
}

fn help_out_the_neighbors(the_neighbors_figs: &mut Garden<Shrivelfig, 6>) {
  // FIXME: I love my neighbors, but their figs are out of control. I'm
  // garden sitting this weekend and want to sabotage watering them.
  // However, my provided recipe isn't type checking and I don't know why.
  the_neighbors_figs.sabotage(shrivel);
}

fn main() {
  help_out_the_neighbors(&mut Garden::new());
}
