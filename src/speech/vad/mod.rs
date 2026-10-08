pub mod vad;
pub use vad::{Segment, VadConfig, detect_speech, segments_from_probabilities};
