use super::frame_params;
use crate::signal::Signal;
use crate::window::{WindowType, get_window};

/*
 * The features extracted from audio files are:
 *      [log_energy, zcr, centroid, rolloff, flatness, energy_std, zcr_std]
 */
pub const NUM_FEATURES: usize = 8;
const MOVING_STD_HALF_WIDTH: usize = 15; // Used in the 'moving_std' function to compute the moving standard deviation of a function
const SPECTRAL_ROLLOFF_PERCENT: f64 = 0.85; // Used in the 'spectral_rolloff' to find the frequency at which SPECTRAL_ROLLOFF_PERCENT of the spectral power falls below that.

pub fn extract_features(signal: &Signal, sample_rate: f64) -> Vec<Vec<f64>> {
    let log_energy = log_energy(signal, sample_rate);
    let zcr = zcr(signal, sample_rate);
    let frames = frame_signal(signal, sample_rate);

    /* How much the energy and zcr differ across MOVING_STD_HALF_WIDTH frames (with each
     * frame separated by a specific hop size), before and after each frame.
     * For instance, if MOVING_STD_HALF_WIDTH=15 and frame_hop=10ms, we
     * compute the standard deviation over the frames extending 150ms on each
     * side of the current frame, which is 300ms total.
     */
    let energy_std = moving_std(&log_energy, MOVING_STD_HALF_WIDTH);
    let zcr_std = moving_std(&zcr, MOVING_STD_HALF_WIDTH);

    frames
        .iter()
        .enumerate()
        .map(|(i, frame)| {
            let spectrum = window_frame(frame).fft();
            let half = spectrum.len() / 2 + 1;
            let power: Vec<f64> = spectrum[..half].iter().map(|x| x.norm_sqr()).collect();

            let centroid = spectral_centroid(&power, sample_rate);
            let rolloff = spectral_rolloff(&power, sample_rate);
            let flatness = spectral_flatness(&power);

            vec![
                log_energy[i],
                zcr[i],
                centroid,
                rolloff,
                flatness,
                energy_std[i],
                zcr_std[i],
            ]
        })
        .collect()
}

/* the following function computes the 'moving standard deviation' of its input.
* Essentially, for each value in its input, it computes the standard deviation of
* that sample plus and minus the elements separated by 'half_width'.
*/
fn moving_std(input: &[f64], half_width: usize) -> Vec<f64> {
    (0..input.len())
        .map(|i| {
            let lo = i.saturating_sub(half_width);
            let hi = (i + half_width + 1).min(input.len());
            let window = &input[lo..hi];
            let mean = window.iter().sum::<f64>() / window.len() as f64;
            let variance =
                window.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / window.len() as f64;
            variance.sqrt()
        })
        .collect()
}

pub fn window_frame(frame: &Signal) -> Signal {
    let window = get_window(WindowType::Han, frame.len());
    frame.clone() * window
}

pub fn frame_signal(signal: &Signal, sample_rate: f64) -> Vec<Signal> {
    /* In function super::frame_params that gets called below, the frame_length and frame_hop variables are defined as:
     * let frame_length = (FRAME_LENGTH_MS * sample_rate / 1000.0) as usize;
     * let frame_hop = (FRAME_HOP_MS * sample_rate / 1000.0) as usize;
     */
    let (frame_length, frame_hop) = frame_params(sample_rate);

    let mut frames = Vec::new();

    let mut start = 0;
    while start + frame_length <= signal.as_slice().len() {
        frames.push(Signal::new(&signal.as_slice()[start..start + frame_length]));
        start += frame_hop;
    }

    frames
}

// Computes the log of the energy for each frame
pub fn log_energy(signal: &Signal, sample_rate: f64) -> Vec<f64> {
    frame_signal(signal, sample_rate)
        .iter()
        .map(|frame| {
            let rms = frame.rms();
            /* For a silent frame, the rms=0 so ln(0) would be undefined.
             * We add a small epsilon to obtain make ln valid (it would give us a very small value instead). */
            (rms * rms + 1e-10).ln()
        })
        .collect()
}

/* Computes the zero crossing rate, i.e., the rate at which the
 * signal crosses the x-axis for each frame.
 */
pub fn zcr(signal: &Signal, sample_rate: f64) -> Vec<f64> {
    frame_signal(signal, sample_rate)
        .iter()
        .map(|frame| frame.zero_crossing_rate())
        .collect()
}

/*
 * Computes the center of gravity for the spectrum.
 * 0.0 means the energy is concentrated at 0 rad/sample.
 * 1.0 means the energy is concentrated at pi rad/sample (or the Nyquist frequency).
 */
pub fn spectral_centroid(power: &[f64], sample_rate: f64) -> f64 {
    let n = power.len();
    let bin_hz = sample_rate / ((n - 1) * 2) as f64;
    let nyquist = sample_rate / 2.0;
    let total = power.iter().sum::<f64>() + 1e-10;

    power
        .iter()
        .enumerate()
        .map(|(k, p)| k as f64 * bin_hz * p)
        .sum::<f64>()
        / total
        / nyquist
}

/*
 * Computs the frequency at which the SPECTRAL_ROLLOFF_PERCENT percent (e.g. 85%)
 * of the total spectral power exists below that.
 */
pub fn spectral_rolloff(power: &[f64], sample_rate: f64) -> f64 {
    let n = power.len();
    let bin_hz = sample_rate / ((n - 1) * 2) as f64;
    let nyquist = sample_rate / 2.0;
    let total = power.iter().sum::<f64>() + 1e-10;

    let mut cumulative = 0.0;

    for (k, p) in power.iter().enumerate() {
        cumulative += p;
        if cumulative >= SPECTRAL_ROLLOFF_PERCENT * total {
            return k as f64 * bin_hz / nyquist;
        }
    }

    1.0
}

/*
 * Spectral flatness computes out noise-like the signal is.
 * If we have a whitenoise, spectral flaness is ~1.0
 * If we have a pure sine wave, the spectral flatness is ~0.0
 */
pub fn spectral_flatness(power: &[f64]) -> f64 {
    let n = power.len() as f64;
    let mean_log = power.iter().map(|p| (p + 1e-10).ln()).sum::<f64>() / n;

    mean_log.exp() / (power.iter().sum::<f64>() / n)
}
