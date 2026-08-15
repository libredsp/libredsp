use libredsp::filter_design::fir_parks_mcclellan::parks_mcclellan_filter_design;
use libredsp::filter_design::parks_mcclellan_filter_design_by_nodes;
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

    /* Design the filter */
    let coefficients =
        parks_mcclellan_filter_design(num_taps, target, weight, LinearPhaseFilterType::I);
    println!("{:?}", coefficients);

    /* Designing a low-pass FIR filter by passing points and let the interplation code handle it */
    let num_taps = 21;
    let desired_target_nodes: Vec<(f64, f64)> = vec![
        (0.0, 1.0),
        (0.2 * std::f64::consts::PI, 1.0),
        (0.25 * std::f64::consts::PI, 0.0),
        (0.5 * std::f64::consts::PI, 0.0),
        (1.0 * std::f64::consts::PI, 1.0),
    ];
    let desired_weighting_nodes = vec![
        (0.0, 1.0),
        (0.6 * std::f64::consts::PI, 1.0),
        (std::f64::consts::PI, 1.0),
    ];

    let coefficients = parks_mcclellan_filter_design_by_nodes(
        num_taps,
        desired_target_nodes,
        desired_weighting_nodes,
        LinearPhaseFilterType::I,
    );
    println!("{:?}", coefficients);
}
