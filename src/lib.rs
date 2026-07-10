use wasm_bindgen::prelude::*;
pub mod signal;
pub mod fft;
pub mod utils;
pub mod types;
pub mod filter_design;
pub mod math;
pub mod spectral_estimation;
pub mod signal_generator;
pub mod simulation;

use crate::filter_design::{
    fir_filter_windowing,
    iir_filter_zero_pole_placement,
    fir_filter_least_squares_linear_phase,
    iir_filter_analog_to_digital
};
use crate::types::*;

fn match_window_type(n: u8) -> WindowType {
    match n {
        0 => WindowType::Rectangular,
        1 => WindowType::Han,
        2 => WindowType::Hamming,
        3 => WindowType::Bartlett,
        _ => panic!("Invalid window type"),
    }
}

fn match_filter_type(n: u8, w1: f64, w2: f64) -> FilterType {
    match n {
        0 => FilterType::Lowpass { w: w1 },
        1 => FilterType::Highpass { w: w1 },
        2 => FilterType::Bandpass { w1, w2 },
        3 => FilterType::Bandstop { w1, w2 },
        _ => panic!("Invalid filter type"),
    }
}

fn match_analog_to_digital_design_type(n: u8) -> AnalogToDigitalTransformationDesignMethod {
    match n {
        0 => AnalogToDigitalTransformationDesignMethod::Butterworth,
        1 => AnalogToDigitalTransformationDesignMethod::Chebyshev,
        _ => panic!("Invalid filter type"),
    }
}

#[wasm_bindgen]
pub fn windowing_method_wasm(
    n: usize, 
    window_type_code: u8,
    filter_type_code: u8,
    w1: f64,
    w2: f64
) -> TransferFunction {
    let filter_type = match_filter_type(filter_type_code, w1, w2);
    let window_type = match_window_type(window_type_code);
    fir_filter_windowing::windowing_method(n, window_type, filter_type)
}

// Receive flattened poles and zeros (in contrast to pairs) to make WASM happy.
#[wasm_bindgen]
pub fn zero_pole_placement_iir_filter_design_wasm(
    poles: Vec<f64>,
    zeros: Vec<f64>,
) -> TransferFunction {
    assert!(poles.len() % 2 == 0);
    assert!(zeros.len() % 2 == 0);

    let poles_pairs: Vec<(f64, f64)> = poles.chunks(2).map(|c| (c[0], c[1])).collect();
    let zeros_pairs: Vec<(f64, f64)> = zeros.chunks(2).map(|c| (c[0], c[1])).collect();

    iir_filter_zero_pole_placement::zero_pole_placement_iir_filter_design(&poles_pairs, &zeros_pairs)
}

#[wasm_bindgen]
pub fn least_squares_linear_phase_fir_wasm(
    f: Vec<f64>,
    a: Vec<f64>,
    weights: Vec<f64>,
    n: usize,
) -> TransferFunction {
    assert!(f.len() == a.len());
    fir_filter_least_squares_linear_phase::least_squares_linear_phase_fir(f, a, weights, n)
}

#[wasm_bindgen]
pub fn iir_filter_analog_to_digital_wasm(
    design_type_code: u8,
    filter_type_code: u8,
    w1: f64,
    w2: f64,
    n: usize,
    chebyshev_coef: f64,
) -> TransferFunction {
    let filter_type = match_filter_type(filter_type_code, w1, w2);
    let design_method = match_analog_to_digital_design_type(design_type_code);    
    iir_filter_analog_to_digital::analog_to_digital_transform_iir_filter_design(design_method, filter_type, n, chebyshev_coef)
}
