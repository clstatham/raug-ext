use std::{
    fmt::Debug,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crossbeam_channel::{Receiver, Sender};
use rand::seq::IndexedRandom;
use raug::prelude::*;
use thiserror::Error;

#[processor(derive(Default))]
pub fn as_bool(#[input] input: &f32, #[output] out: &mut bool) -> ProcResult<()> {
    *out = input.as_bool();
    Ok(())
}

#[processor(derive(Default))]
pub fn as_float(#[input] input: &bool, #[output] out: &mut f32) -> ProcResult<()> {
    *out = if *input { 1.0 } else { 0.0 };
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
pub enum ChannelError {
    #[error("Failed to send message")]
    SendError,
    #[error("Failed to receive message")]
    ReceiveError(#[from] crossbeam_channel::TryRecvError),
}

#[processor(derive(Debug))]
pub fn tx<T>(#[state] tx: &mut Sender<T>, #[input] input: &T) -> ProcResult<()>
where
    T: Signal,
{
    if tx.try_send(input.clone()).is_err() {
        return Err(ProcessorError::new(ChannelError::SendError));
    }
    Ok(())
}

#[processor(derive(Debug))]
pub fn rx<T>(
    #[state] rx: &mut Receiver<T>,
    #[state] last: &mut T,
    #[output] out: &mut T,
) -> ProcResult<()>
where
    T: Signal,
{
    if let Ok(value) = rx.try_recv() {
        last.clone_from(&value);
    }
    out.clone_from(last);
    Ok(())
}

#[derive(Debug)]
pub struct Channel<T: Signal> {
    tx: Arc<Mutex<Tx<T>>>,
    rx: Arc<Mutex<Rx<T>>>,
}

impl<T: Signal> Channel<T> {
    pub fn new(init: T) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Channel::from_tx_rx(
            Tx {
                tx,
                input: init.clone(),
                _marker0: PhantomData,
            },
            Rx {
                rx,
                last: init.clone(),
                _marker0: PhantomData,
            },
        )
    }

    pub(crate) fn from_tx_rx(tx: Tx<T>, rx: Rx<T>) -> Self {
        Channel {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    #[inline]
    pub fn send(&self, value: T) -> Result<(), ChannelError> {
        let tx = self.tx.lock().unwrap();
        if tx.tx.try_send(value).is_err() {
            return Err(ChannelError::SendError);
        }
        Ok(())
    }

    #[inline]
    pub fn recv(&self) -> T {
        let mut rx = self.rx.lock().unwrap();
        if let Ok(value) = rx.rx.try_recv() {
            rx.last.clone_from(&value);
        }
        rx.last.clone()
    }
}

impl<T: Signal> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Channel {
            tx: Arc::clone(&self.tx),
            rx: Arc::clone(&self.rx),
        }
    }
}

impl<T: Signal + Default> Default for Channel<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Signal> Processor for Channel<T> {
    fn name(&self) -> &str {
        "Channel"
    }

    fn input_spec(&self) -> Vec<SignalSpec> {
        vec![SignalSpec::new("input", T::signal_type())]
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
        for sample_index in 0..inputs.block_size() {
            outputs.set_output_as(0, sample_index, &self.recv())?;
        }

        Ok(())
    }
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
