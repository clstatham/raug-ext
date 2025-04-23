use raug::prelude::*;
use raug_ext::prelude::*;

fn main() {
    let graph = Graph::new();

    let clock = Metro::from_tempo_and_ticks(144.0, 4) // 144bpm, 4 ticks per beat
        .node(&graph, (), ());

    let bd_pat = BoolPattern::new("x . . . x . . . x . . . x . . . ").node(&graph, &clock);
    let bd = OneShot::load("examples/assets/bd.wav")
        .unwrap()
        .node(&graph, &bd_pat[0], ());

    let sd_pat = BoolPattern::new(". . . . x . . . . . . . x . . x ").node(&graph, &clock);
    let sd_vel_pat = IntPattern::new([2, 2, 1]).node(&graph, &sd_pat[0]);
    let sd = OneShot::load("examples/assets/sd.wav")
        .unwrap()
        .node(&graph, &sd_pat[0], ());
    let sd = &sd[0] * sd_vel_pat[0].cast::<i64, f32>();

    let mix = &bd[0] + &sd[0];

    graph.dac(&mix);
    graph.dac(&mix);

    graph
        .play_until(CpalStream::default(), || {
            std::io::stdin().read_line(&mut String::new()).ok();
            true
        })
        .unwrap()
        .join()
        .unwrap();
}
