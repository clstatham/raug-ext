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

#[derive(Clone, Copy, Default, PartialEq)]
enum AdsrState {
    #[default]
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[processor(derive(Default))]
pub fn adsr(
    env: ProcEnv,
    #[state] state: &mut AdsrState,
    #[state] value: &mut f32,
    #[state] time: &mut f32,

    #[input] trig: &bool,
    #[input] attack: &f32,
    #[input] decay: &f32,
    #[input] sustain: &f32,
    #[input] release: &f32,

    #[output] out: &mut f32,
) -> ProcResult<()> {
    match *state {
        AdsrState::Off => {
            *value = 0.0;
            *time = 0.0;
            if *trig {
                *state = AdsrState::Attack;
            }
        }
        AdsrState::Attack => {
            if !trig {
                *time = 0.0;
                *state = AdsrState::Release;
            } else if *time >= *attack {
                *value = 1.0;
                *time = 0.0;
                *state = AdsrState::Decay;
            } else {
                *value = (*time / *attack).min(1.0);
            }
        }
        AdsrState::Decay => {
            if !trig {
                *time = 0.0;
                *state = AdsrState::Release;
            } else if *time >= *decay {
                *value = *sustain;
                *time = 0.0;
                *state = AdsrState::Sustain;
            } else {
                *value = 1.0 - (*time / *decay).min(1.0);
            }
        }
        AdsrState::Sustain => {
            if !*trig {
                *time = 0.0;
                *state = AdsrState::Release;
            } else {
                *value = *sustain;
                *time = 0.0;
            }
        }
        AdsrState::Release => {
            if *trig {
                *value = 0.0;
                *time = 0.0;
                *state = AdsrState::Attack;
            } else if *time >= *release {
                *state = AdsrState::Off;
            } else {
                *value = (*time / *release).min(1.0);
            }
        }
    }

    *time += env.sample_rate.recip();
    *out = *value;

    Ok(())
}

impl Adsr {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            state: AdsrState::Off,
            value: 0.0,
            time: 0.0,
            trig: false,
            attack,
            decay,
            sustain,
            release,
        }
    }
}

pub trait IntoPattern<T: Signal> {
    fn into_pattern(self) -> List<T>;
}

impl<T: Signal> IntoPattern<T> for List<T> {
    fn into_pattern(self) -> List<T> {
        self
    }
}

impl<T: Signal> IntoPattern<T> for &[T] {
    fn into_pattern(self) -> List<T> {
        List::from_slice(self)
    }
}

impl<T: Signal, const N: usize> IntoPattern<T> for [T; N] {
    fn into_pattern(self) -> List<T> {
        List::from_slice(&self)
    }
}

impl IntoPattern<bool> for &str {
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
    #[state] pattern: &mut List<bool>,
    #[state] index: &mut usize,
    #[input] trig: &bool,
    #[output] out: &mut bool,
    #[output] length: &mut i64,
) -> ProcResult<()> {
    *length = pattern.len() as i64;

    if !pattern.is_empty() {
        if *trig {
            *out = pattern[*index];
            *index += 1;
            *index %= pattern.len();
        } else {
            *out = false;
        }
    }

    Ok(())
}

impl BoolPattern {
    pub fn new(pat: impl IntoPattern<bool>) -> Self {
        Self {
            pattern: pat.into_pattern(),
            index: 0,
            trig: false,
        }
    }
}

impl IntoPattern<i64> for &str {
    fn into_pattern(self) -> List<i64> {
        let mut list = List::default();
        for slc in self.split_ascii_whitespace() {
            match slc.parse() {
                Ok(i) => list.push(i),
                Err(e) => panic!("Invalid pattern element `{slc}`: {e}"),
            }
        }
        list
    }
}

#[processor(derive(Default))]
pub fn int_pattern(
    #[state] pattern: &mut List<i64>,
    #[state] index: &mut usize,
    #[input] trig: &bool,
    #[output] out: &mut i64,
    #[output] length: &mut i64,
) -> ProcResult<()> {
    *length = pattern.len() as i64;

    if !pattern.is_empty() {
        *out = pattern[*index];
        if *trig {
            *index += 1;
            *index %= pattern.len();
        }
    }

    Ok(())
}

impl IntPattern {
    pub fn new(pat: impl IntoPattern<i64>) -> Self {
        let pattern = pat.into_pattern();
        let index = pattern.len() - 1;
        Self {
            pattern,
            index,
            trig: false,
        }
    }
}
