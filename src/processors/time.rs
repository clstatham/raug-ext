use raug::prelude::*;

#[processor(derive(Default))]
pub fn metro(
    env: ProcEnv,
    #[state] last_time: &mut f32,
    #[state] next_time: &mut f32,
    #[state] time: &mut f32,
    #[input] period: &f32,
    #[input] reset: &bool,
    #[output] out: &mut bool,
) -> ProcResult<()> {
    if *reset {
        *last_time = 0.0;
        *next_time = 0.0;
        *time = 0.0;
    }

    *out = if *time >= *next_time {
        *last_time = *time;
        *next_time = *time + (*period * env.sample_rate);
        true
    } else {
        false
    };

    *time += 1.0;

    Ok(())
}

impl Metro {
    /// Constructs a new [`Metro`] processor with the given period between ticks.
    pub fn new(period: f32) -> Self {
        Self {
            last_time: 0.0,
            next_time: 0.0,
            time: 0.0,
            period,
            reset: false,
        }
    }

    /// Constructs a new [`Metro`] processor from the given tempo (in BPM, or beats per minute) and TPB (number of ticks per beat).
    /// The metronome will tick `bpm * tpb` times per minute.
    pub fn from_tempo_and_ticks(bpm: f32, tpb: usize) -> Self {
        assert_ne!(tpb, 0, "Ticks per beat mut be >= 1");
        Self::new(60.0 / bpm / (tpb as f32))
    }
}

#[processor(derive(Default))]
pub fn decay(
    env: ProcEnv,
    #[state] last_trig: &mut bool,
    #[state] value: &mut f32,
    #[state] time: &mut f32,

    #[input] trig: &bool,
    #[input] tau: &f32,

    #[output] out: &mut f32,
) -> ProcResult<()> {
    let tau = tau.max(0.0);

    if *trig && !*last_trig {
        *value = 1.0;
        *time = 0.0;
    } else if *value > 0.0 {
        *time += 1.0 / env.sample_rate;
        *value = (-tau.recip() * *time).exp();
    }

    *last_trig = *trig;
    *value = value.clamp(0.0, 1.0);
    *out = *value;

    Ok(())
}

impl Decay {
    pub fn new(tau: f32) -> Self {
        Self {
            last_trig: false,
            value: 0.0,
            time: 0.0,
            trig: false,
            tau,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
enum AdsrState {
    Attack,
    Decay,
    #[default]
    Sustain,
    Release,
}

#[processor]
pub fn adsr(
    env: ProcEnv,
    #[state] state: &mut AdsrState,
    #[state] last_gate: &mut bool,
    #[state] value: &mut f32,

    #[input] gate: &bool,
    #[input] attack: &f32,
    #[input] decay: &f32,
    #[input] sustain: &f32,
    #[input] release: &f32,

    #[output] out: &mut f32,
) -> ProcResult<()> {
    let attack = attack * env.sample_rate;
    let decay = decay * env.sample_rate;
    let release = release * env.sample_rate;

    if *gate && !*last_gate {
        *value = 0.0;
        *state = AdsrState::Attack;
    } else if !*gate && *last_gate {
        *state = AdsrState::Release;
    }

    let slope = match *state {
        AdsrState::Attack => {
            if attack > 0.0 {
                1.0 / attack
            } else {
                1.0
            }
        }
        AdsrState::Decay => {
            if decay > 0.0 {
                -(1.0 - *sustain) / decay
            } else {
                -1.0
            }
        }
        AdsrState::Sustain => 0.0,
        AdsrState::Release => {
            if release > 0.0 {
                -(1.0 - *sustain) / release
            } else {
                -1.0
            }
        }
    };

    *value += slope;

    if *state == AdsrState::Attack && *value >= 1.0 {
        *value = 1.0;
        *state = AdsrState::Decay;
    } else if *state == AdsrState::Decay && *value <= *sustain {
        *value = *sustain;
        *state = AdsrState::Sustain;
    } else if *state == AdsrState::Release && *value <= 0.0 {
        *value = 0.0;
        *state = AdsrState::Sustain;
    }

    *last_gate = *gate;
    *out = *value;

    Ok(())
}

impl Default for Adsr {
    fn default() -> Self {
        Self {
            state: AdsrState::Sustain,
            last_gate: false,
            value: 0.0,
            gate: false,
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
    }
}

impl Adsr {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            ..Default::default()
        }
    }
}

pub trait IntoPattern<T: Signal> {
    fn into_pattern(self) -> List<T>;
}

impl<T: Signal> IntoPattern<T> for List<T> {
    #[inline]
    fn into_pattern(self) -> List<T> {
        self
    }
}

impl<T: Signal> IntoPattern<T> for &[T] {
    #[inline]
    fn into_pattern(self) -> List<T> {
        List::from_slice(self)
    }
}

impl<T: Signal, const N: usize> IntoPattern<T> for [T; N] {
    #[inline]
    fn into_pattern(self) -> List<T> {
        List::from_slice(&self)
    }
}

impl IntoPattern<bool> for &str {
    #[inline]
    fn into_pattern(self) -> List<bool> {
        let mut list = List::with_capacity(self.len());
        for slc in self.split_ascii_whitespace() {
            match slc {
                "." => list.push(false),
                _ => list.push(true),
            }
        }
        list
    }
}

#[processor(derive(Default))]
pub fn bool_pattern(
    #[state] index_state: &mut usize,
    #[input] trig: &bool,
    #[input] pattern: &Str,
    #[output] out: &mut bool,
    #[output] index: &mut f32,
) -> ProcResult<()> {
    let pattern = pattern.into_pattern();
    *index = *index_state as f32;

    if !pattern.is_empty() {
        if *trig {
            *out = pattern[*index_state];
            *index_state += 1;
            *index_state %= pattern.len();
        } else {
            *out = false;
        }
    }

    Ok(())
}

impl IntoPattern<f32> for &str {
    #[inline]
    fn into_pattern(self) -> List<f32> {
        let mut list = List::with_capacity(self.len());
        for slc in self.split_ascii_whitespace() {
            if let Ok(i) = slc.parse() {
                list.push(i)
            }
        }
        list
    }
}

impl<const N: usize> IntoPattern<f32> for [i32; N] {
    fn into_pattern(self) -> List<f32> {
        let mut list = List::default();
        for n in self {
            list.push(n as f32);
        }
        list
    }
}

#[processor]
pub fn pattern(
    #[state] index_state: &mut isize,
    #[input] trig: &bool,
    #[input] pattern: &Str,
    #[output] out: &mut f32,
    #[output] index: &mut f32,
) -> ProcResult<()> {
    let pattern = pattern.into_pattern();
    *index = *index_state as f32;

    if !pattern.is_empty() {
        if *index_state < 0 {
            *out = pattern[(pattern.len() as isize + *index_state) as usize];
        } else {
            *out = pattern[*index_state as usize];
        }
        if *trig {
            *index_state += 1;
            *index_state %= pattern.len() as isize;
        }
    }

    Ok(())
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            index_state: -1,
            trig: false,
            pattern: Str::new(),
        }
    }
}
