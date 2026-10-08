use crate::noise_reduction::spectral_subtraction;
use crate::speech::{read_wav_mono, write_wav_mono};
use std::path::Path;

pub fn stft_background_noise_removal(
    input: &Path,
    output: &Path,
    noise_start: f64,
    noise_end: f64,
    frame_len_ms: f64,
    alpha: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let audio = read_wav_mono(input)?;
    let sample_rate = audio.spec.sample_rate as f64;
    let signal = audio.signal;

    let mut frame_size = (frame_len_ms / 1000.0 * sample_rate) as usize;

    if frame_size == 0 {
        return Err("frame length is too small".into());
    }

    if frame_size % 2 != 0 {
        frame_size += 1;
    }

    let noise_start_sample = (noise_start * sample_rate) as usize;
    let noise_end_sample = (noise_end * sample_rate) as usize;

    let clean_signal = spectral_subtraction(
        &signal,
        frame_size,
        noise_start_sample,
        noise_end_sample,
        alpha,
    )?;

    write_wav_mono(&output, &clean_signal, audio.spec)?;

    Ok(())
}
