mod signal;
mod fft;
mod utils;
use signal::Signal;
pub mod types;
pub mod filter_design;
pub mod math;
use crate::filter_design::{
    iir_filter_analog_to_digital::*,
    iir_filter_zero_pole_placement::*,
    fir_filter_least_squares_linear_phase::*,
    fir_filter_windowing::*
};
use crate::types::*;

fn main() {
    let coefs = windowing_method(6, types::WindowType::Rectangular, types::FilterType::Lowpass{w: 0.5});
    println!("coef {:?}", coefs);

    // Create a signal
    let signal = Signal::new(vec![1.0, 0.0, 0.0]);
    println!("Signal: {:?}", signal.clone().to_vec());
    
    // Apply FFT
    let spectrum = signal.fft();
    println!("Result of FFT: {:?}", spectrum);

    // Filter design examples
    let coefs_iir: TransferFunction = analog_to_digital_transform_iir_filter_design(
        AnalogToDigitalTransformationDesignMethod::Butterworth, 
        FilterType::Highpass { w: 0.5 }, 
        3,
        0.2);
    println!("coef {:?}", coefs_iir);

    let fir_ls_lp = least_squares_linear_phase_fir(
        vec![0.0, 0.15, 0.85, 1.0], 
        vec![1.0, 1.0, 0.0, 0.0],
        vec![1.0, 100.0],
        11);
    println!("coef {:?}", fir_ls_lp);
    println!("");

    let iir_pole_placement = zero_pole_placement_iir_filter_design(
        &vec![(0.47, -0.495), (0.47, 0.495)],
        &vec![(-0.469, -0.605), (-0.469, 0.605)],
    );
    println!("coef {:?}", iir_pole_placement);

}