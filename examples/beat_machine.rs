use raug::prelude::*;
use raug_ext::prelude::*;

fn main() {
    let graph = Graph::new();

    let clock = Metro::from_tempo_and_ticks(144.0, 4) // 144bpm, 4 ticks per beat
        .node(&graph, (), ());

    let bd_pat = Pattern::new("x . . . x . . . x . . . x . . . ").node(&graph, &clock);
    let bd = OneShot::load("examples/assets/bd.wav")
        .unwrap()
        .node(&graph, &bd_pat[0], ());

    let sd_pat = Pattern::new(". . . . x . . . . . . . x . . x ").node(&graph, &clock);
    let sd = OneShot::load("examples/assets/sd.wav")
        .unwrap()
        .node(&graph, &sd_pat[0], ());

    let mix = bd[0].clone() + sd[0].clone();
    let mix = mix * 0.5;

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
