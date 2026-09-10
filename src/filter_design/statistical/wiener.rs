use crate::signal::*;
use nalgebra::{DMatrix, DVector};

pub fn wiener(x: &Signal, d: &Signal, filter_order: usize) -> Signal {
    assert_eq!(x.len(), d.len());
    assert!(filter_order > 0);
    assert!(x.len() >= filter_order);

    let m = filter_order;

    let r_lags = x.autocorrelation(m - 1);
    let p_lags = x.crosscorrelation(d, m - 1);

    let mut r = DMatrix::<f64>::zeros(m, m);

    for i in 0..m {
        for j in 0..m {
            r[(i, j)] = r_lags[(i as isize - j as isize).unsigned_abs() as usize];
        }
    }

    let p = DVector::<f64>::from_vec(p_lags);

    // Rw = p
    // We could explicitly compute R⁻¹ and multiply by p,
    // but it is numerically better to solve Rw = p directly
    // using LU decomposition.
    let w = r
        .lu()
        .solve(&p)
        .expect("Wiener equation could not be solved");

    Signal::new(w.iter().copied().collect())
}
