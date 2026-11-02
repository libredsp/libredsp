use libredsp::filter_design::frequency_selective::*;
use libredsp::types::*;
use std::f64::consts::PI;
use libredsp::window::*;
fn main() {
    /* Windowing method: */

    let coefs = windowing_method(6, WindowType::Rectangular, FilterType::Lowpass { w: 0.5 });
    println!("coef {:?}", coefs);

    /* Analog-to-digital transform: */

    let coefs_iir: TransferFunction = analog_to_digital_transform_iir_filter_design(
        AnalogToDigitalTransformationDesignMethod::Butterworth,
        FilterType::Highpass { w: 0.5 },
        3,
        0.2,
    );
    println!("coef {:?}", coefs_iir);

    let fir_ls_lp = least_squares_linear_phase_fir(
        vec![0.0, 0.15, 0.85, 1.0],
        vec![1.0, 1.0, 0.0, 0.0],
        vec![1.0, 100.0],
        11,
    );
    println!("coef {:?}", fir_ls_lp);
    println!("");

    /* Zero-pole placement method: */
    let iir_pole_placement = zero_pole_placement_iir_filter_design(
        &vec![(0.47, -0.495), (0.47, 0.495)],
        &vec![(-0.469, -0.605), (-0.469, 0.605)],
    );
    println!("coef {:?}", iir_pole_placement);

    /* Parks-McClellan: */
    let num_taps = 11;
    let target = |f: f64| {
        if f <= 0.18 * PI { 1.0 } else { 0.0 }
    };

    let weight = |f: f64| {
        if f <= 0.18 * PI { 1.0 } else { 10.0 }
    };

    let coefficients =
        parks_mcclellan_filter_design(num_taps, target, weight, LinearPhaseFilterType::I);
    println!("{:?}", coefficients);
}
