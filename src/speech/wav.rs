use crate::signal::Signal;
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::path::Path;

pub struct Audio {
    pub signal: Signal,
    pub spec: WavSpec,
}

// Reads a WAV file with any number of channels (mono, stereo, ...)
pub fn read_wav(input: &Path) -> Result<Audio, Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(input)?;
    let spec = reader.spec();

    let samples: Vec<f64> = match spec.sample_format {
        SampleFormat::Int => {
            let scale = 1.0 / (1_u64 << (spec.bits_per_sample - 1)) as f64;
            reader
                .samples::<i32>()
                .map(|s| s.map(|x| x as f64 * scale))
                .collect::<Result<_, _>>()?
        }
        SampleFormat::Float => reader
            .samples::<f32>()
            .map(|s| s.map(|x| x as f64))
            .collect::<Result<_, _>>()?,
    };

    Ok(Audio {
        signal: Signal::new(samples),
        spec,
    })
}

pub fn read_wav_mono(input: &Path) -> Result<Audio, Box<dyn std::error::Error>> {
    let mut audio = read_wav(input)?;
    let channels = audio.spec.channels as usize;

    if channels == 0 {
        return Err("WAV file has zero channels".into());
    }

    let mono: Vec<f64> = audio
        .signal
        .to_vec()
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f64>() / channels as f64)
        .collect();

    audio.signal = Signal::new(mono);
    audio.spec.channels = 1;

    Ok(audio)
}

pub fn write_wav(
    output: &Path,
    signal: &Signal,
    spec: WavSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = WavWriter::create(output, spec)?;

    for sample in signal.iter() {
        match spec.sample_format {
            SampleFormat::Int => {
                let max = (1_i64 << (spec.bits_per_sample - 1)) - 1;
                let value = (sample.clamp(-1.0, 1.0) * max as f64) as i32;
                writer.write_sample(value)?;
            }
            SampleFormat::Float => writer.write_sample(sample as f32)?,
        }
    }

    writer.finalize()?;
    Ok(())
}

pub fn write_wav_mono(
    output: &Path,
    signal: &Signal,
    mut spec: WavSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    spec.channels = 1;
    write_wav(output, signal, spec)
}
