use num_complex::Complex;
use crate::{
    math::convolve::convolve,
    types::TransferFunction
};

pub fn zero_pole_placement_iir_filter_design(
    poles: &[(f64, f64)],
    zeros: &[(f64, f64)],
) -> TransferFunction {

    assert!(poles.len() % 2 == 0,"Number of poles must be even!");
    assert!(zeros.len() % 2 == 0, "Number of zeros must be even!");

    let mut num_as_product_of_factors: Vec<Vec<Complex<f64>>> = Vec::new();
    let mut den_as_product_of_factors: Vec<Vec<Complex<f64>>> = Vec::new();

    for i in 0..zeros.len() {
        num_as_product_of_factors.push(vec![
            Complex::new(1.0, 0.0),
            Complex::new(-zeros[i].0, -zeros[i].1)
        ]);
    }

    for i in 0..poles.len() {
        den_as_product_of_factors.push(vec![
            Complex::new(1.0, 0.0),
            Complex::new(-poles[i].0, -poles[i].1)
        ]);
    }

    // Construct numerator polynomial
    let num_complex = if num_as_product_of_factors.is_empty() {
        vec![Complex::new(1.0, 0.0)]
    } else {
        let mut tmp = num_as_product_of_factors[0].clone();
        for i in 1..num_as_product_of_factors.len() {
            tmp = convolve(&tmp, &num_as_product_of_factors[i]);
        }
        tmp
    };

    // Construct denominator polynomial
    let den_complex = if den_as_product_of_factors.is_empty() {
        vec![Complex::new(1.0, 0.0)]
    } else {
        let mut tmp = den_as_product_of_factors[0].clone();
        for i in 1..den_as_product_of_factors.len() {
            tmp = convolve(&tmp, &den_as_product_of_factors[i]);
        }
        tmp
    };

    // Extract real coefficients. They should already be pretty close to real numbers.
    let num: Vec<f64> =
        num_complex.iter()
        .map(|c| c.re)
        .collect();

    let den: Vec<f64> =
        den_complex.iter()
        .map(|c| c.re)
        .collect();

    TransferFunction {
        num,
        den,
    }
}