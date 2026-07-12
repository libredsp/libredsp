use nalgebra::{DMatrix};

pub fn tf_to_ss(
    num: &[f64],
    den: &[f64],
) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>, f64) {
    if num.is_empty() || den.len() < 2 {
        panic!("Invalid transfer function.");
    }

    let (num, den) = if den[0] != 1.0 {
        normalize_tf(num, den)
    } else {
        (num.to_vec(), den.to_vec())
    };

    let n = den.len() - 1;

    let mut a = DMatrix::zeros(n, n);
    // Shift states
    for i in 0..n - 1 {
        a[(i, i + 1)] = 1.0;
    }

    // Last row
    for i in 0..n {
        a[(n - 1, i)] = -den[n - i];
    }

    // B
    let mut b = DMatrix::zeros(n, 1);
    b[(n - 1, 0)] = 1.0;

    // C
    let mut c = DMatrix::zeros(1, n);
    for i in 0..n {
        c[(0, i)] = *num.get(n - 1 - i).unwrap_or(&0.0);
    }

    let d = 0.0;

    (a, b, c, d)
}

fn normalize_tf(num: &[f64], den: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let leading = den[0];
    let num: Vec<f64> = num.iter().map(|&e| e / leading).collect();
    let den: Vec<f64> = den.iter().map(|&e| e / leading).collect();
    (num, den)
}