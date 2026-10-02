use std::error::Error;

use crate::signal::Signal;

/// Read a WAV file as a mono signal in [-1, 1] and its sample rate.
pub fn read_wav(path: &str) -> Result<(Signal, f64), Box<dyn Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let channels = spec.channels as usize;

    let samples: Vec<f64> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .map(|s| s.map(|v| v as f64))
            .collect::<Result<_, _>>()?,
        hound::SampleFormat::Int => {
            let scale = 1.0 / (1u64 << (spec.bits_per_sample - 1)) as f64;
            reader
                .samples::<i32>()
                .map(|s| s.map(|v| v as f64 * scale))
                .collect::<Result<_, _>>()?
        }
    };

    // Average the channels to get mono
    let mono: Vec<f64> = samples
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f64>() / channels as f64)
        .collect();

    Ok((Signal::new(mono), spec.sample_rate as f64))
}
