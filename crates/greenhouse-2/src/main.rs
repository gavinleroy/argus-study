#[cfg(test)]
mod tests;

use brew::prelude::*;

async fn shrivel(i1: Shrivelfig, i2: Shrivelfig, i3: Dittany) -> Pink {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .boil()
    .rest()
    .pour_as()
}

// TASK: My neighbors have a garden of shrivelfigs that keeps growing, 
// and growing, and GROWING! It's getting out of control. This weekend I plan 
// to restrict their growth by slipping them a little concoction I've whipped up. 
// The recipe is simple, it returns a pink potion that takes 2 parts shrivelfig, 
// and 1 part dittany. The secret: It must be boiled.
fn help_out_the_neighbors(the_neighbors_figs: &mut Garden<Shrivelfig, 6>) {
  the_neighbors_figs.sabotage(shrivel);
}

fn main() {
  help_out_the_neighbors(&mut Garden::new());
}
