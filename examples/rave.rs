use raug::prelude::*;
use raug_ext::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn supersaw(
    graph: &Graph,
    pitch: impl IntoOutput,
    detune: impl IntoOutput,
    gate: impl IntoOutput,
    attack: impl IntoOutputOpt,
    decay: impl IntoOutputOpt,
    sustain: impl IntoOutputOpt,
    release: impl IntoOutputOpt,
) -> Node {
    let pitch = pitch.into_output(graph);
    let gate = gate.into_output(graph);
    let detune = detune.into_output(graph);

    let freq1 = PitchToFreq::default().node(graph, &pitch + &detune);
    let freq2 = PitchToFreq::default().node(graph, &pitch - &detune);

    let saw1 = BlSawOscillator::default().node(graph, freq1);
    let saw2 = BlSawOscillator::default().node(graph, freq2);

    let saws = (saw1 + saw2) * 0.5;

    let adsr = Adsr::default().node(graph, gate, attack, decay, sustain, release);
    saws * adsr
}

fn main() {
    env_logger::init();

    let graph = Graph::new(0, 2);

    let clock = Metro::from_tempo_and_ticks(144.0, 4) // 144bpm, 4 ticks per beat
        .node(&graph, (), ());

    let bd_pat = BoolPattern::default().node(&graph, &clock, "x . . . x . . . x . . . x . . . ");
    let bd = OneShot::load("examples/assets/bd.wav")
        .unwrap()
        .node(&graph, &bd_pat[0], ());

    let sd_pat = BoolPattern::default().node(&graph, &clock, ". . . . x . . . . . . . x . . x ");
    let sd_vel_pat = Pattern::default().node(&graph, &sd_pat[0], "2 2 1");
    let sd = OneShot::load("examples/assets/sd.wav")
        .unwrap()
        .node(&graph, &sd_pat[0], ());
    let sd = &sd[0] * &sd_vel_pat[0];

    let saw_pat = BoolPattern::default().node(&graph, &clock, "x . x . x . x .");
    let base = 40;
    let saw_notes = Pattern::default().node(&graph, &saw_pat[0], "0 3 7");
    let saw_notes = &saw_notes[0] + base;
    let saw = supersaw(
        &graph,
        &saw_notes[0],
        0.1,
        saw_pat[0].trig_to_gate(0.1),
        0.0,
        0.2,
        0.0,
        1.0,
    );

    let mix = &bd[0] + &sd[0] + &saw[0] * 0.2;

    let master = PeakLimiter::default().node(&graph, mix, (), (), ());

    graph.dac((&master, &master));

    graph.allocate(48000.0, 512);

    let stream = CpalOut::spawn(&AudioBackend::Default, &AudioDevice::Default)
        .record_to_wav("recording.wav");
    graph.play_for(stream, Duration::from_secs(10)).unwrap();
}
