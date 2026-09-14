use crate::{
    signal::Signal,
    window::{WindowType, get_window},
};

use num_complex::Complex;

type Complex64 = Complex<f64>;

pub fn spectral_subtraction(
    signal: &Signal,
    frame_size: usize,
    noise_start_sample: usize,
    noise_end_sample: usize,
    alpha: f64,
) -> Result<Signal, Box<dyn std::error::Error>> {
    let total_samples = signal.len();

    // Some checks
    if frame_size == 0 {
        return Err("frame size must be greater than zero".into());
    }

    if frame_size % 2 != 0 {
        return Err("frame size must be even".into());
    }

    if total_samples < frame_size {
        return Err("audio is shorter than one analysis frame".into());
    }

    if noise_start_sample >= total_samples {
        return Err("noise interval starts after the end of the audio".into());
    }

    if noise_end_sample > total_samples {
        return Err("noise interval extends beyond the end of the audio".into());
    }

    if noise_end_sample <= noise_start_sample {
        return Err("invalid noise interval".into());
    }

    let hop_size = frame_size / 2;

    // Get Han window
    let window = get_window(WindowType::Han, frame_size);

    // Divide 'signal' into overlapping frames and apply window on each frame
    let frames = get_frames(signal, frame_size, hop_size, &window);

    // Apply FFT on frames
    let spectra: Vec<Vec<Complex64>> = frames.iter().map(|frame| frame.fft()).collect();

    let fft_size = spectra[0].len();

    // Record magnitude and phase of the FFT
    let mut magnitudes = Vec::with_capacity(spectra.len());
    let mut phases = Vec::with_capacity(spectra.len());

    for spectrum in &spectra {
        magnitudes.push(spectrum.iter().map(|x| x.norm()).collect::<Vec<_>>());

        phases.push(spectrum.iter().map(|x| x.arg()).collect::<Vec<_>>());
    }

    // Get noise profile
    let noise_profile = get_noise_profile(
        signal,
        &window,
        noise_start_sample,
        noise_end_sample,
        frame_size,
        hop_size,
        fft_size,
    )?;

    // Subtract spectrum
    let cleaned_magnitudes: Vec<Vec<f64>> = magnitudes
        .iter()
        .map(|magnitude| {
            magnitude
                .iter()
                .zip(&noise_profile)
                .map(|(&m, &noise)| (m - alpha * noise).max(0.0))
                .collect()
        })
        .collect();

    // Reconstruct full spectrum
    let mut reconstructed_frames = Vec::with_capacity(frames.len());

    for i in 0..frames.len() {
        let mut spectrum = Vec::with_capacity(fft_size);

        for k in 0..fft_size {
            spectrum.push(Complex64::from_polar(
                cleaned_magnitudes[i][k],
                phases[i][k],
            ));
        }

        reconstructed_frames.push(Signal::ifft(&spectrum));
    }

    // Overlap-add
    let clean_signal = overlap_add(&reconstructed_frames, &window, hop_size, frame_size);

    Ok(clean_signal.slice(0, total_samples))
}

fn get_frames(signal: &Signal, frame_size: usize, hop_size: usize, window: &Signal) -> Vec<Signal> {
    let total_samples = signal.len();
    let num_frames = (total_samples - 1) / hop_size + 1;

    let mut frames = Vec::with_capacity(num_frames);

    for i in 0..num_frames {
        let start = i * hop_size;
        let end = (start + frame_size).min(total_samples);

        let frame = get_frame(signal, start, end, frame_size, window);
        frames.push(frame);
    }

    frames
}

fn get_frame(
    signal: &Signal,
    start: usize,
    end: usize,
    frame_size: usize,
    window: &Signal,
) -> Signal {
    let mut frame = if end - start < frame_size {
        let mut data = signal.slice(start, end);
        data.zero_pad(frame_size - data.len());
        data
    } else {
        signal.slice(start, end)
    };

    frame = frame * window;
    frame
}

fn get_noise_profile(
    signal: &Signal,
    window: &Signal,
    noise_start_sample: usize,
    noise_end_sample: usize,
    frame_size: usize,
    hop_size: usize,
    fft_size: usize,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let noise_signal = signal.slice(noise_start_sample, noise_end_sample);
    let noise_length = noise_signal.len();

    if noise_length < frame_size {
        return Err("noise interval is shorter than one analysis frame".into());
    }

    let noise_num_frames = (noise_length - frame_size) / hop_size + 1;
    let mut noise_profile = vec![0.0; fft_size];

    for i in 0..noise_num_frames {
        let start = i * hop_size;

        let frame = noise_signal.slice(start, start + frame_size) * window;
        let spectrum = frame.fft();

        for k in 0..fft_size {
            noise_profile[k] += spectrum[k].norm();
        }
    }

    for value in &mut noise_profile {
        *value /= noise_num_frames as f64;
    }

    Ok(noise_profile)
}

fn overlap_add(frames: &[Signal], window: &Signal, hop_size: usize, frame_size: usize) -> Signal {
    let out_len = (frames.len() - 1) * hop_size + frame_size;

    let mut clean_signal = Signal::new(vec![0.0; out_len]);
    let mut window_sum = Signal::new(vec![0.0; out_len]);

    for (i, frame) in frames.iter().enumerate() {
        let start = i * hop_size;

        for n in 0..frame_size {
            clean_signal[start + n] += frame[n];
            window_sum[start + n] += window[n];
        }
    }

    for i in 0..out_len {
        if window_sum[i] >= 1e-6 {
            clean_signal[i] /= window_sum[i];
        }
    }

    clean_signal
}
