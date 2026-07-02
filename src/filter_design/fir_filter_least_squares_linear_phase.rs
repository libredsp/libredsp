use nalgebra::{DMatrix, DVector};
use std::f64::consts::PI;
use crate::types::*;

pub fn least_squares_linear_phase_fir(
    f: Vec<f64>,
    a: Vec<f64>,
    weights: Vec<f64>,
    n: usize,
) -> TransferFunction {
    let l = 4098;
    let m = (n - 1) / 2;
    let normalized_freqs: Vec<f64> = (0..=l).map(|i| i as f64 / l as f64).collect();
    let w: Vec<f64> = normalized_freqs.iter().map(|&x| x * PI).collect();

    // Linearly interpolates desired amplitude inside each band
    let mut d: Vec<f64> = vec![0.0; l + 1];
    for k in 0..f.len() / 2 {
        let f1 = f[k * 2];
        let f2 = f[k * 2 + 1];
        let a1 = a[k * 2];
        let a2 = a[k * 2 + 1];

        for i in 0..=l {
            if normalized_freqs[i] >= f1 && normalized_freqs[i] <= f2 {
                d[i] = a1 + (a2 - a1) * (normalized_freqs[i] - f1) / (f2 - f1);
            }
        }
    }

    // Build cosine matrix
    let c = DMatrix::from_fn(l + 1, m + 1, |i, k| (w[i] * k as f64).cos());
    
    // Compute weight vector
    let mut wvec: Vec<f64> = Vec::new();
    let mut weights_idx = 0;
    for i in 0..=l {
        let freq = normalized_freqs[i];
        while weights_idx < weights.len() - 1 && freq > f[2 * weights_idx + 1] {
            weights_idx += 1;
        }
        wvec.push(weights[weights_idx]);
    }
    
    // Apply weights (element-wise multiplication with diagonal matrix)
    let w_diag = DMatrix::from_diagonal(&DVector::from_vec(wvec));
    let wc = &w_diag * &c;
    let wd = &w_diag * DVector::from_vec(d);
    
    // Solve a = inv(C' W C) C' W D
    let ct_wc = c.transpose() * wc;
    let ct_wd = c.transpose() * wd;
    // TODO use inverse
    let a = ct_wc.try_inverse().unwrap() * ct_wd;
    
    // Build impulse response
    let a_vec = a.as_slice();
    let mut h = vec![0.0; n];
    h[m] = a_vec[0];
    for k in 1..=m {
        h[m - k] = a_vec[k] / 2.0;
        h[m + k] = a_vec[k] / 2.0;
    }
    
    TransferFunction {
        num: h,
        den: vec![1.0],
    }
}