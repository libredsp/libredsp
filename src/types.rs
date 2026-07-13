// src/types.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TransferFunction {

    #[wasm_bindgen(skip)]
    pub num: Vec<f64>,

    #[wasm_bindgen(skip)]
    pub den: Vec<f64>,
}

#[wasm_bindgen]
impl TransferFunction {

    #[wasm_bindgen(getter)]
    pub fn num(&self) -> Vec<f64> {
        self.num.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn den(&self) -> Vec<f64> {
        self.den.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowType {
    Rectangular,
    Bartlett,
    Hamming,
    Han,
    Kaiser { min_stopband_attinuation: f64, transition_width: f64},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    Lowpass {w: f64},
    Highpass {w: f64},
    Bandpass {w1: f64, w2: f64},
    Bandstop {w1: f64, w2: f64},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnalogToDigitalTransformationDesignMethod {
    Butterworth,
    Chebyshev,
}

