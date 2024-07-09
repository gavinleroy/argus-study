use brew::prelude::*;

async fn shrivel(i1: Shrivelfig) -> impl Potion {
  EmptyCauldron::new().mix(i1).boil().rest().pour_as::<Pink>()
}

fn help_out_the_neighbors(the_neighbors_figs: &mut Garden<Shrivelfig, 6>) {
  the_neighbors_figs.sabotage(shrivel);
}

fn main() {
  help_out_the_neighbors(&mut Garden::new());
}
