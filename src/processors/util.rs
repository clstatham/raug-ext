use std::marker::PhantomData;

use crossbeam_channel::{Receiver, Sender};
use rand::seq::IndexedRandom;
use raug::prelude::*;

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

#[derive(Clone)]
pub struct Select<T: Signal + Default> {
    pub arity: usize,
    _marker: PhantomData<T>,
}

impl<T: Signal + Default> Select<T> {
    pub fn new(arity: usize) -> Self {
        Select {
            arity,
            _marker: PhantomData,
        }
    }
}

impl<T: Signal + Default> Processor for Select<T> {
    fn input_spec(&self) -> Vec<SignalSpec> {
        vec![
            SignalSpec::new("input", T::signal_type()),
            SignalSpec::new("index", i64::signal_type()),
        ]
    }

    fn output_spec(&self) -> Vec<SignalSpec> {
        (0..self.arity)
            .map(|i| SignalSpec::new(format!("out_{}", i), T::signal_type()))
            .collect()
    }

    fn create_output_buffers(&self, size: usize) -> Vec<AnyBuffer> {
        (0..self.arity)
            .map(|_| AnyBuffer::zeros::<T>(size))
            .collect()
    }

    fn process(
        &mut self,
        inputs: ProcessorInputs,
        mut outputs: ProcessorOutputs,
    ) -> Result<(), ProcessorError> {
        let Some(input) = inputs.input_as::<T>(0) else {
            return Ok(());
        };

        let Some(index) = inputs.input_as::<i64>(1) else {
            return Ok(());
        };

        for (sample_index, index) in index.iter().enumerate() {
            if *index < 0 || *index >= self.arity as i64 {
                continue;
            }

            let output_index = *index as usize;

            if sample_index < input.len() {
                outputs.set_output_as(output_index, sample_index, &input[sample_index])?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Merge<T: Signal + Default> {
    pub arity: usize,
    _marker: PhantomData<T>,
}

impl<T: Signal + Default> Merge<T> {
    pub fn new(arity: usize) -> Self {
        Merge {
            arity,
            _marker: PhantomData,
        }
    }
}

impl<T: Signal + Default> Processor for Merge<T> {
    fn input_spec(&self) -> Vec<SignalSpec> {
        let mut inputs = Vec::with_capacity(self.arity + 1);
        inputs.push(SignalSpec::new("index", i64::signal_type()));
        for i in 0..self.arity {
            inputs.push(SignalSpec::new(format!("input_{}", i), T::signal_type()));
        }
        inputs
    }

    fn output_spec(&self) -> Vec<SignalSpec> {
        vec![SignalSpec::new("out", T::signal_type())]
    }

    fn create_output_buffers(&self, size: usize) -> Vec<AnyBuffer> {
        vec![AnyBuffer::zeros::<T>(size)]
    }

    fn process(
        &mut self,
        inputs: ProcessorInputs,
        mut outputs: ProcessorOutputs,
    ) -> Result<(), ProcessorError> {
        let Some(index) = inputs.input_as::<i64>(0) else {
            return Ok(());
        };

        for (sample_index, index) in index.iter().enumerate() {
            if *index < 0 || *index >= self.arity as i64 {
                continue;
            }

            let input_index = *index as usize + 1;
            let input = inputs.input_as::<T>(input_index).unwrap();

            if sample_index < input.len() {
                outputs.set_output_as(0, sample_index, &input[sample_index])?;
            }
        }

        Ok(())
    }
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

#[processor]
pub fn tx<T>(#[state] tx: &mut Sender<T>, #[input] input: &T) -> ProcResult<()>
where
    T: Signal + Default,
{
    if tx.try_send(input.clone()).is_err() {
        return Err(ProcessorError::ProcessingError(
            "Failed to send message".to_string(),
        ));
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

pub fn signal_channel<T: Signal + Default>() -> (Tx<T>, Rx<T>) {
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
