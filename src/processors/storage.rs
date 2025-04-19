use raug::prelude::*;

#[processor(allocate = sample_allocate)]
#[allow(unused)]
pub fn sample(
    env: ProcEnv,
    #[state] buf: &mut Vec<f32>,
    #[state] sample_rate: &mut f32,
    #[input] index: &f32,
    #[input] wrap: &bool,
    #[output] out: &mut f32,
    #[output] length: &mut f32,
) -> ProcResult<()> {
    let index = if *wrap {
        *index % buf.len() as f32
    } else {
        *index
    };
    let floor = index.floor() as usize;
    let ceil = index.ceil() as usize;
    let t = index - floor as f32;
    let floor_value = if floor < buf.len() { buf[floor] } else { 0.0 };
    let ceil_value = if ceil < buf.len() { buf[ceil] } else { 0.0 };
    let value = floor_value * (1.0 - t) + ceil_value * t;
    *out = value;

    *length = buf.len() as f32;

    Ok(())
}

fn resample_linear(input: &[f32], input_rate: f32, output_rate: f32) -> Vec<f32> {
    samplerate::convert(
        input_rate as u32,
        output_rate as u32,
        1,
        samplerate::ConverterType::SincFastest,
        input,
    )
    .unwrap()
}

fn sample_allocate(proc: &mut Sample, sample_rate: f32, _block_size: usize) {
    if sample_rate <= 0.0 || sample_rate == proc.sample_rate {
        return;
    }
    proc.buf = resample_linear(&proc.buf, proc.sample_rate, sample_rate);
    proc.sample_rate = sample_rate;
}

impl Sample {
    pub fn load(path: &str) -> Result<Self, hound::Error> {
        let reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        let channels = spec.channels as usize;
        let sample_rate = spec.sample_rate as f32;
        let buf = match spec.sample_format {
            hound::SampleFormat::Int => {
                let buf = reader
                    .into_samples::<i32>()
                    .step_by(channels) // take only one channel
                    .collect::<Result<Vec<_>, _>>()?;
                buf.into_iter()
                    .map(|s| s as f32 / (1 << spec.bits_per_sample) as f32)
                    .collect()
            }
            hound::SampleFormat::Float => {
                reader
                    .into_samples::<f32>()
                    .step_by(channels) // take only one channel
                    .collect::<Result<Vec<_>, _>>()?
            }
        };

        Ok(Self {
            buf,
            sample_rate,
            index: 0.0,
            wrap: false,
        })
    }

    pub fn length(&self) -> f32 {
        self.buf.len() as f32
    }
}
