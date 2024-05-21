//! A safe space to cultivate exotic plants.

use brew::prelude::*;

async fn brighten_leaves(
    _: I<P3, Pinch, Dittany>,
    _: I<P4, Dash, Bubotuber>,
    _: I<P7, Scoop, Aconite>
)
{}

async fn strengthen_bark(
    _: I<P3, Scoop, Dittany>,
    _: I<P4, Pinch, Alihotsy>,
    _: I<P6, Heaped<Scoop>, Shrivelfig>
)
{}

async fn water_roots(
    _: I<P3, Scoop, Bubotuber>,
    _: I<P4, Scoop, Alihotsy>,
    _: I<P7, Scoop, Aconite>,
    _: I<P4, Scoop, Alihotsy>,
    _: I<P3, Scoop, Dittany>,
)
{}

fn test_1() {
    Garden::<Bubotuber, _>::new(P12::new())
        .add_schedule(Delta::<Minutely, P128>::new(), brighten_leaves)
        .garden()
}

fn test_2() {
    Garden::<Wiggentree, _>::new(P1::new())
        .add_schedule(Delta::<Yearly, P1>::new(), strengthen_bark)
        .garden()
}

fn test_3() {
    Garden::<Shrivelfig, _>::new(P10::new())
        .add_schedule(Delta::<Daily, P15>::new(), water_roots)
        .garden()
}

fn main() {
    test_1();
    test_2();
    test_3();
}
