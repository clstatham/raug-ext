use std::{fmt::Debug, marker::PhantomData};

use crossbeam_channel::{Receiver, Sender};
use rand::seq::IndexedRandom;
use raug::prelude::*;
use thiserror::Error;

pub trait CastTo<T> {
    fn cast(&self) -> T;
}

impl CastTo<f32> for f64 {
    fn cast(&self) -> f32 {
        *self as f32
    }
}

impl CastTo<f64> for f32 {
    fn cast(&self) -> f64 {
        *self as f64
    }
}

impl CastTo<i64> for f64 {
    fn cast(&self) -> i64 {
        *self as i64
    }
}

impl CastTo<i64> for f32 {
    fn cast(&self) -> i64 {
        *self as i64
    }
}

impl CastTo<f32> for i64 {
    fn cast(&self) -> f32 {
        *self as f32
    }
}

#[processor(derive(Default))]
pub fn cast<T, U>(#[input] a: &T, #[output] out: &mut U) -> ProcResult<()>
where
    T: Signal + CastTo<U>,
    U: Signal + Default,
{
    *out = a.cast();
    Ok(())
}

#[processor(derive(Default))]
pub fn sample_rate(env: ProcEnv, #[output] out: &mut f32) -> ProcResult<()> {
    *out = env.sample_rate;
    Ok(())
}

#[processor(derive(Default))]
pub fn message<T>(
    #[state] last_message: &mut T,
    #[input] trig: &bool,
    #[input] message: &Option<T>,
    #[output] out: &mut Option<T>,
) -> ProcResult<()>
where
    T: Signal,
{
    if let Some(msg) = message.as_ref() {
        last_message.clone_from(msg);
    }

    if *trig {
        *out = Some(last_message.clone());
    } else {
        *out = None;
    }

    Ok(())
}

impl<T: Signal> Message<T> {
    pub fn new(msg: T) -> Self {
        Message {
            last_message: msg,
            trig: false,
            message: None,
            _marker0: PhantomData,
        }
    }
}

#[processor(derive(Default))]
pub fn register<T>(
    #[state] last_value: &mut Option<T>,
    #[input] set: &Option<T>,
    #[input] clear: &bool,
    #[output] out: &mut Option<T>,
) -> ProcResult<()>
where
    T: Signal,
{
    if let Some(value) = set.as_ref() {
        *last_value = Some(value.clone());
    }

    if *clear {
        *last_value = None;
    }

    *out = last_value.clone();
    Ok(())
}

#[processor(derive(Default))]
pub fn unwrap_or<T>(#[input] a: &Option<T>, #[input] b: &T, #[output] out: &mut T) -> ProcResult<()>
where
    T: Signal + Default,
{
    if let Some(value) = a {
        *out = value.clone();
    } else {
        *out = b.clone();
    }
    Ok(())
}

#[processor(derive(Default))]
pub fn sample_and_hold<T>(
    #[state] last_value: &mut T,
    #[input] input: &T,
    #[input] trig: &bool,
    #[output] out: &mut T,
) -> ProcResult<()>
where
    T: Signal + Default,
{
    if *trig {
        *last_value = input.clone();
    }

    *out = last_value.clone();
    Ok(())
}

#[processor(derive(Default))]
pub fn some<T>(#[input] a: &T, #[output] out: &mut Option<T>) -> ProcResult<()>
where
    T: Signal,
{
    *out = Some(a.clone());
    Ok(())
}

#[processor(derive(Default))]
pub fn random_choice<T>(
    #[state] state: &mut Option<T>,
    #[input] trig: &bool,
    #[input] options: &List<T>,
    #[output] out: &mut Option<T>,
) -> ProcResult<()>
where
    T: Signal,
{
    if *trig {
        *state = options.as_ref().choose(&mut rand::rng()).cloned();
    }

    *out = state.clone();
    Ok(())
}

#[derive(Error, Debug)]
pub enum ChannelError<T: Signal + Debug> {
    #[error("Failed to send message")]
    SendError(#[from] crossbeam_channel::TrySendError<T>),
    #[error("Failed to receive message")]
    ReceiveError(#[from] crossbeam_channel::TryRecvError),
}

#[processor]
pub fn tx<T>(#[state] tx: &mut Sender<T>, #[input] input: &T) -> ProcResult<()>
where
    T: Signal + Default + Debug,
{
    if let Err(e) = tx.try_send(input.clone()) {
        return Err(ProcessorError::new(ChannelError::SendError(e)));
    }
    Ok(())
}

#[processor]
pub fn rx<T>(#[state] rx: &mut Receiver<T>, #[output] out: &mut T) -> ProcResult<()>
where
    T: Signal + Default,
{
    match rx.try_recv() {
        Ok(value) => *out = value,
        Err(_) => *out = T::default(),
    }
    Ok(())
}

pub fn signal_channel<T: Signal + Default + Debug>() -> (Tx<T>, Rx<T>) {
    let (tx, rx) = crossbeam_channel::unbounded();
    (
        Tx {
            tx,
            input: T::default(),
            _marker0: PhantomData,
        },
        Rx {
            rx,
            _marker0: PhantomData,
        },
    )
}

#[processor(derive(Default))]
pub fn toggle(
    #[state] state: &mut bool,
    #[input] trig: &bool,
    #[output] out: &mut bool,
) -> ProcResult<()> {
    if *trig {
        *state = !*state;
    }

    *out = *state;

    Ok(())
}

#[processor(derive(Default))]
pub fn trig_to_gate(
    env: ProcEnv,
    #[state] t: &mut f32,
    #[state] last_trig: &mut bool,
    #[state] state: &mut bool,
    #[input] trig: &bool,
    #[input] length: &f32,
    #[output] gate: &mut bool,
) -> ProcResult<()> {
    let length_samples = *length * env.sample_rate;
    if *trig && !*last_trig {
        // rising edge
        *t = 0.0;
        *state = true;
    } else if !*trig && *last_trig {
        // falling edge
        *t = 0.0;
    }

    if *state {
        *state = *t < length_samples;
        *t += 1.0;
    }

    *gate = *state;
    *last_trig = *trig;

    Ok(())
}
