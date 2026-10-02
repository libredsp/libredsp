use crate::signal::Signal;

use super::features::extract_features;
use super::frame_params;
use super::logistic_regression::LogisticRegression;

/// Split a recording into speech and non-speech segments.
pub fn detect_speech(
    model: &LogisticRegression,
    signal: &Signal,
    sample_rate: f64,
    config: &VadConfig,
) -> Vec<Segment> {
    let (_, hop) = frame_params(sample_rate);
    let hop_seconds = hop as f64 / sample_rate;
    let total_seconds = signal.len() as f64 / sample_rate;

    let features = extract_features(signal, sample_rate);
    let probabilities = model.predict_proba_all(&features);

    segments_from_probabilities(&probabilities, hop_seconds, total_seconds, config)
}

/// A time interval labeled as speech or not.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub is_speech: bool,
}

pub struct VadConfig {
    pub threshold: f64,
    pub smoothing_frames: usize,
    /// Speech shorter than this is dropped (seconds).
    pub min_speech: f64,
    /// Gaps between speech shorter than this are filled in (seconds).
    pub min_gap: f64,
}

impl Default for VadConfig {
    fn default() -> Self {
        VadConfig {
            threshold: 0.6,
            smoothing_frames: 11,
            min_speech: 0.5,
            min_gap: 0.5,
        }
    }
}

/// Runs of equal values as (value, start, end), end exclusive.
fn runs(decisions: &[bool]) -> Vec<(bool, usize, usize)> {
    let mut result = Vec::new();
    let mut start = 0;
    for i in 1..=decisions.len() {
        if i == decisions.len() || decisions[i] != decisions[start] {
            result.push((decisions[start], start, i));
            start = i;
        }
    }
    result
}

/// Flip runs of `value` shorter than `min_len` frames to the opposite value.
/// With `keep_edges`, runs at the start or end of the recording are left alone.
fn flip_short_runs(decisions: &mut [bool], value: bool, min_len: usize, keep_edges: bool) {
    for (v, start, end) in runs(decisions) {
        if v != value || end - start >= min_len {
            continue;
        }
        if keep_edges && (start == 0 || end == decisions.len()) {
            continue;
        }
        for d in &mut decisions[start..end] {
            *d = !value;
        }
    }
}

/// Turn per frame speech probabilities into segments covering the recording.
pub fn segments_from_probabilities(
    probabilities: &[f64],
    hop_seconds: f64,
    total_seconds: f64,
    config: &VadConfig,
) -> Vec<Segment> {
    let n = probabilities.len();
    let half = config.smoothing_frames / 2;

    // Average each probability with its neighbors, then apply the threshold
    let mut decisions: Vec<bool> = (0..n)
        .map(|i| {
            let lo = i.saturating_sub(half);
            let hi = (i + half + 1).min(n);
            let average = probabilities[lo..hi].iter().sum::<f64>() / (hi - lo) as f64;
            average >= config.threshold
        })
        .collect();

    let min_gap = (config.min_gap / hop_seconds).round() as usize;
    let min_speech = (config.min_speech / hop_seconds).round() as usize;
    flip_short_runs(&mut decisions, false, min_gap, true);
    flip_short_runs(&mut decisions, true, min_speech, false);

    // Group consecutive frames with the same decision into segments
    let mut segments = Vec::new();
    let mut start = 0;
    for i in 1..=n {
        if i == n || decisions[i] != decisions[start] {
            segments.push(Segment {
                start: start as f64 * hop_seconds,
                end: i as f64 * hop_seconds,
                is_speech: decisions[start],
            });
            start = i;
        }
    }

    // Make the segments cover the whole recording
    if let Some(first) = segments.first_mut() {
        first.start = 0.0;
    }
    if let Some(last) = segments.last_mut() {
        last.end = total_seconds;
    }

    segments
}
