use num_complex::Complex;
use std::f64::consts::PI;

pub fn get_causal_butterworth_poles(n: usize, omega_c: f64) -> Vec<Complex<f64>> {
    let mut poles = Vec::new();
    // Iterating from 0..n generates only the causal (left half-plane) Butterworth poles.
    // Alternatively, iterate from 0..2n to generate all roots,
    // then filter using p.re < 0.0 to retain only the causal poles.
    for k in 0..n {
        let theta = PI * (2.0 * k as f64 + n as f64 + 1.0) / (2.0 * n as f64);
        poles.push(Complex::new(omega_c * theta.cos(), omega_c * theta.sin()));
    }

    poles
}

pub fn get_chebyshev_i_poles(n: usize, omega_c: f64, epsilon: f64) -> Vec<Complex<f64>> {
    let mut poles = Vec::new();
    let beta = (1.0 / n as f64) * (1.0 / epsilon).asinh();
    let sigma = beta.sinh();
    let omega = beta.cosh();

    for k in 1..=n {
        let theta: f64 = (PI * (2.0 * k as f64 - 1.0)) / (2.0 * n as f64);
        let re = -sigma * theta.sin();
        let im = omega * theta.cos();
        poles.push(Complex::new(omega_c * re, omega_c * im));
    }
    return poles;
}