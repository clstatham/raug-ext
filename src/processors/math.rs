use raug::prelude::*;

#[processor(derive(Default))]
pub fn powf(#[input] a: &f32, #[input] b: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.powf(*b);
    Ok(())
}

#[processor(derive(Default))]
pub fn powi(#[input] a: &f32, #[input] b: &i64, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.powi(*b as i32);
    Ok(())
}

#[processor(derive(Default))]
pub fn sqrt(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.sqrt();
    Ok(())
}

#[processor(derive(Default))]
pub fn cbrt(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.cbrt();
    Ok(())
}

#[processor(derive(Default))]
pub fn exp(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.exp();
    Ok(())
}

#[processor(derive(Default))]
pub fn exp2(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.exp2();
    Ok(())
}

#[processor(derive(Default))]
pub fn ln(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.ln();
    Ok(())
}

#[processor(derive(Default))]
pub fn log(#[input] a: &f32, #[input] b: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.log(*b);
    Ok(())
}

#[processor(derive(Default))]
pub fn log2(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.log2();
    Ok(())
}

#[processor(derive(Default))]
pub fn log10(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.log10();
    Ok(())
}

#[processor(derive(Default))]
pub fn sin(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.sin();
    Ok(())
}

#[processor(derive(Default))]
pub fn cos(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.cos();
    Ok(())
}

#[processor(derive(Default))]
pub fn tan(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.tan();
    Ok(())
}

#[processor(derive(Default))]
pub fn asin(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.asin();
    Ok(())
}

#[processor(derive(Default))]
pub fn acos(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.acos();
    Ok(())
}

#[processor(derive(Default))]
pub fn atan(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.atan();
    Ok(())
}

#[processor(derive(Default))]
pub fn atan2(#[input] a: &f32, #[input] b: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.atan2(*b);
    Ok(())
}

#[processor(derive(Default))]
pub fn hypot(#[input] a: &f32, #[input] b: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.hypot(*b);
    Ok(())
}

#[processor(derive(Default))]
pub fn sinh(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.sinh();
    Ok(())
}

#[processor(derive(Default))]
pub fn cosh(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.cosh();
    Ok(())
}

#[processor(derive(Default))]
pub fn tanh(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.tanh();
    Ok(())
}

#[processor(derive(Default))]
pub fn asinh(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.asinh();
    Ok(())
}

#[processor(derive(Default))]
pub fn acosh(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.acosh();
    Ok(())
}

#[processor(derive(Default))]
pub fn atanh(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.atanh();
    Ok(())
}

#[processor(derive(Default))]
pub fn abs(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.abs();
    Ok(())
}

#[processor(derive(Default))]
pub fn signum(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.signum();
    Ok(())
}

#[processor(derive(Default))]
pub fn floor(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.floor();
    Ok(())
}

#[processor(derive(Default))]
pub fn ceil(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.ceil();
    Ok(())
}

#[processor(derive(Default))]
pub fn round(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.round();
    Ok(())
}

#[processor(derive(Default))]
pub fn trunc(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.trunc();
    Ok(())
}

#[processor(derive(Default))]
pub fn fract(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.fract();
    Ok(())
}

#[processor(derive(Default))]
pub fn recip(#[input] a: &f32, #[output] out: &mut f32) -> ProcResult<()> {
    *out = a.recip();
    Ok(())
}

#[processor(derive(Default))]
pub fn max<T>(#[input] a: &T, #[input] b: &T, #[output] out: &mut T) -> ProcResult<()>
where
    T: std::cmp::PartialOrd + Signal + Default,
{
    out.clone_from(if a > b { a } else { b });
    Ok(())
}

#[processor(derive(Default))]
pub fn min<T>(#[input] a: &T, #[input] b: &T, #[output] out: &mut T) -> ProcResult<()>
where
    T: std::cmp::PartialOrd + Signal + Default,
{
    out.clone_from(if a < b { a } else { b });
    Ok(())
}

#[processor(derive(Default))]
pub fn clamp<T>(
    #[input] a: &T,
    #[input] min: &T,
    #[input] max: &T,
    #[output] out: &mut T,
) -> ProcResult<()>
where
    T: std::cmp::PartialOrd + Signal + Default,
{
    out.clone_from(if a < min {
        min
    } else if a > max {
        max
    } else {
        a
    });
    Ok(())
}

#[processor(derive(Default))]
pub fn lerp(
    #[input] a: &f32,
    #[input] b: &f32,
    #[input] t: &f32,
    #[output] out: &mut f32,
) -> ProcResult<()> {
    *out = *a + (*b - *a) * *t;
    Ok(())
}

#[processor(derive(Default))]
pub fn smooth_step(
    #[input] edge0: &f32,
    #[input] edge1: &f32,
    #[input] x: &f32,
    #[output] out: &mut f32,
) -> ProcResult<()> {
    let t = (*x - *edge0) / (*edge1 - *edge0);
    *out = t.clamp(0.0, 1.0).powf(3.0) * (t * (t * 6.0 - 15.0) + 10.0);
    Ok(())
}
