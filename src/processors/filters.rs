use raug::prelude::*;
use std::f32::consts::PI;

#[processor]
pub fn lowpass1(
    env: ProcEnv,
    #[state] prev_out: &mut f32,
    #[input] input: &f32,
    #[input] cutoff: &f32,
    #[output] out: &mut f32,
) -> ProcResult<()> {
    let coeff = 1.0 - (-2.0 * PI * cutoff / env.sample_rate).exp();
    *prev_out = coeff * input + (1.0 - coeff) * *prev_out;
    *out = *prev_out;
    Ok(())
}

#[processor]
pub fn highpass1(
    env: ProcEnv,
    #[state] prev_out: &mut f32,
    #[state] prev_in: &mut f32,
    #[input] input: &f32,
    #[input] cutoff: &f32,
    #[output] out: &mut f32,
) -> ProcResult<()> {
    let coeff = (-2.0 * PI * cutoff / env.sample_rate).exp();
    *prev_out = coeff * (*prev_out + input - *prev_in);
    *prev_in = *input;
    *out = *prev_out;
    Ok(())
}
