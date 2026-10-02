use super::frame_params;
use crate::signal::Signal;
use crate::window::{WindowType, get_window};

pub const NUM_FEATURES: usize = 8;

/// Standard deviation of `values` in a window of `half_width` frames on
/// each side of every index (clipped at the edges).
fn rolling_std(values: &[f64], half_width: usize) -> Vec<f64> {
    (0..values.len())
        .map(|i| {
            let lo = i.saturating_sub(half_width);
            let hi = (i + half_width + 1).min(values.len());
            let window = &values[lo..hi];
            let mean = window.iter().sum::<f64>() / window.len() as f64;
            let variance =
                window.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / window.len() as f64;
            variance.sqrt()
        })
        .collect()
}

/// One feature vector per frame:
/// [log_energy, zcr, centroid, rolloff, flatness, voice_band_ratio,
///  energy_std, zcr_std]
pub fn extract_features(signal: &Signal, sample_rate: f64) -> Vec<Vec<f64>> {
    let time_domain = energy_and_zcr(signal, sample_rate);
    let spectral = spectral_features(signal, sample_rate);

    // How much energy and zcr vary over about 150 ms on each side
    let energy: Vec<f64> = time_domain.iter().map(|t| t.0).collect();
    let zcr: Vec<f64> = time_domain.iter().map(|t| t.1).collect();
    let energy_std = rolling_std(&energy, 15);
    let zcr_std = rolling_std(&zcr, 15);

    time_domain
        .iter()
        .zip(&spectral)
        .enumerate()
        .map(|(i, (&(e, z), s))| vec![e, z, s[0], s[1], s[2], s[3], energy_std[i], zcr_std[i]])
        .collect()
}

pub fn window_frame(frame: &Signal) -> Signal {
    let window = get_window(WindowType::Han, frame.len());

    frame.clone() * window
}

pub fn frame_signal(signal: &Signal, sample_rate: f64) -> Vec<Signal> {
    // let frame_length = (FRAME_LENGTH_MS * sample_rate / 1000.0) as usize;
    // let frame_hop = (FRAME_HOP_MS * sample_rate / 1000.0) as usize;
    let (frame_length, frame_hop) = frame_params(sample_rate);

    let mut frames = Vec::new();

    let mut start = 0;
    while start + frame_length <= signal.as_slice().len() {
        frames.push(Signal::new(&signal.as_slice()[start..start + frame_length]));
        start += frame_hop;
    }

    frames
}

/// (log energy, zero crossing rate) for every frame.
pub fn energy_and_zcr(signal: &Signal, sample_rate: f64) -> Vec<(f64, f64)> {
    frame_signal(signal, sample_rate)
        .iter()
        .map(|frame| {
            let rms = frame.rms();
            let log_energy = (rms * rms + 1e-10).ln(); /* We add the 1e-10 value to prevent ln(0) being undef. */
            (log_energy, frame.zero_crossing_rate())
        })
        .collect()
}

/// (centroid, rolloff, flatness, voice_band_ratio) for every frame.
/// Centroid and rolloff are divided by Nyquist, so they lie between 0 and 1.
pub fn spectral_features(signal: &Signal, sample_rate: f64) -> Vec<[f64; 4]> {
    frame_signal(signal, sample_rate)
        .iter()
        .map(|frame| {
            let spectrum = window_frame(frame).fft();
            let half = spectrum.len() / 2 + 1;
            let bin_hz = sample_rate / spectrum.len() as f64;
            let nyquist = sample_rate / 2.0;

            // Power of the non-negative frequency bins
            let power: Vec<f64> = spectrum[..half].iter().map(|x| x.norm_sqr()).collect();
            let total = power.iter().sum::<f64>() + 1e-10;

            // Centroid: power weighted mean frequency
            let centroid = power
                .iter()
                .enumerate()
                .map(|(k, p)| k as f64 * bin_hz * p)
                .sum::<f64>()
                / total
                / nyquist;

            // Rolloff: frequency below which 85% of the power lies
            let mut cumulative = 0.0;
            let mut rolloff = 1.0;
            for (k, p) in power.iter().enumerate() {
                cumulative += p;
                if cumulative >= 0.85 * total {
                    rolloff = k as f64 * bin_hz / nyquist;
                    break;
                }
            }

            // Flatness: geometric mean / arithmetic mean (near 1 = noise like)
            let mean_log = power.iter().map(|p| (p + 1e-10).ln()).sum::<f64>() / half as f64;
            let flatness = mean_log.exp() / (total / half as f64);

            // Share of power between 300 and 3400 Hz
            let voice_band: f64 = power
                .iter()
                .enumerate()
                .filter(|(k, _)| {
                    let f = *k as f64 * bin_hz;
                    f >= 300.0 && f < 3400.0
                })
                .map(|(_, p)| p)
                .sum();

            [centroid, rolloff, flatness, voice_band / total]
        })
        .collect()
}
