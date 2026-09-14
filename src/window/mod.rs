#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowType {
    Rectangular,
    Bartlett,
    Hamming,
    Han,
    Kaiser {
        min_stopband_attinuation: f64,
        transition_width: f64,
    },
}

pub mod window;
pub use window::*;
