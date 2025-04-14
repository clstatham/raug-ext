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
    U: Signal,
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
        *last_message = *msg;
    }

    if *trig {
        *out = Some(*last_message);
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
            out: None,
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
        *last_value = Some(*value);
    }

    if *clear {
        *last_value = None;
    }

    *out = *last_value;
    Ok(())
}

#[processor(derive(Default))]
pub fn or<T>(
    #[input] a: &Option<T>,
    #[input] b: &Option<T>,
    #[output] out: &mut Option<T>,
) -> ProcResult<()>
where
    T: Signal,
{
    if let Some(value) = a {
        *out = Some(*value);
    } else if let Some(value) = b {
        *out = Some(*value);
    } else {
        *out = None;
    }
    Ok(())
}

#[processor(derive(Default))]
pub fn unwrap_or<T>(#[input] a: &Option<T>, #[input] b: &T, #[output] out: &mut T) -> ProcResult<()>
where
    T: Signal,
{
    if let Some(value) = a {
        *out = *value;
    } else {
        *out = *b;
    }
    Ok(())
}

#[derive(Clone)]
pub struct Select<T: Signal> {
    pub arity: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Signal> Select<T> {
    pub fn new(arity: usize) -> Self {
        Select {
            arity,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Signal> Processor for Select<T> {
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

    fn create_output_buffers(&self, size: usize) -> Vec<ErasedBuffer> {
        (0..self.arity)
            .map(|_| ErasedBuffer::zeros::<T>(size))
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
                outputs.set_output_as(output_index, sample_index, input[sample_index])?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Merge<T: Signal> {
    pub arity: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Signal> Merge<T> {
    pub fn new(arity: usize) -> Self {
        Merge {
            arity,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Signal> Processor for Merge<T> {
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

    fn create_output_buffers(&self, size: usize) -> Vec<ErasedBuffer> {
        vec![ErasedBuffer::zeros::<T>(size)]
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
                outputs.set_output_as(0, sample_index, input[sample_index])?;
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
    T: Signal,
{
    if *trig {
        *last_value = *input;
    }

    *out = *last_value;
    Ok(())
}

#[processor(derive(Default))]
pub fn some<T>(#[input] a: &T, #[output] out: &mut Option<T>) -> ProcResult<()>
where
    T: Signal,
{
    *out = Some(*a);
    Ok(())
}
