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
    pub fn new(period: f32) -> Self {
        Self {
            last_time: 0.0,
            next_time: 0.0,
            time: 0.0,
            period,
            reset: false,
        }
    }
}

#[processor(derive(Default))]
pub fn decay_env(
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

impl DecayEnv {
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
