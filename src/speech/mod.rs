pub mod features;

pub(crate) const FRAME_LENGTH_MS: f64 = 25.0;
pub(crate) const FRAME_HOP_MS: f64 = 10.0;

pub fn frame_params(sample_rate: f64) -> (usize, usize) {
    let frame_length = (FRAME_LENGTH_MS * sample_rate / 1000.0) as usize;
    let frame_hop = (FRAME_HOP_MS * sample_rate / 1000.0) as usize;
    (frame_length, frame_hop)
}

pub mod logistic_regression;
pub use logistic_regression::{LogisticRegression, Metrics, TrainConfig};

pub mod vad;
pub use vad::{Segment, VadConfig, detect_speech, segments_from_probabilities};

pub mod dataset;
pub mod wav;
