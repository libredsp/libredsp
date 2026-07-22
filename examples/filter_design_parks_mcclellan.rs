use libredsp::filter_design::fir_parks_mcclellan::parks_mcclellan_filter_design;
use libredsp::types::LinearPhaseFilterType;
use std::f64::consts::PI;

fn main() {
    /* Designing a low-pass FIR filter */
    let num_taps = 11;
    let target = |f: f64| {
        if f <= 0.18 * PI { 1.0 } else { 0.0 }
    };

    let weight = |f: f64| {
        if f <= 0.18 * PI { 1.0 } else { 10.0 }
    };

    // Design the filter
    let coefficients =
        parks_mcclellan_filter_design(num_taps, target, weight, LinearPhaseFilterType::I);
    println!("{:?}", coefficients);
}
