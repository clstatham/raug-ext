use raug::prelude::*;
use raug_ext::prelude::*;

fn main() {
    let graph = Graph::new();

    let clock = Metro::new(60.0 / 120.0 / 4.0).node(&graph, (), ()); // 144bpm, 4 ticks per beat

    let bd_pat = Pattern::new("x...x...x...x...").node(&graph, &clock);
    let bd = OneShot::load("examples/assets/bd.wav")
        .unwrap()
        .node(&graph, &bd_pat[0], ());

    let sd_pat = Pattern::new("....x.......x..x").node(&graph, &clock);
    let sd = OneShot::load("examples/assets/sd.wav")
        .unwrap()
        .node(&graph, &sd_pat[0], ());

    let mix = bd[0].clone() + sd[0].clone();
    let mix = mix * 0.5;

    graph.dac(&mix);
    graph.dac(&mix);

    let mut stream = CpalStream::default();
    stream.spawn(&graph).unwrap();
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(60));
    stream.stop().unwrap();
    stream.join().unwrap();
}
