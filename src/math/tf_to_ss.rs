use nalgebra::{DMatrix};

pub fn tf_to_ss(num: &[f64], den: &[f64]) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>, f64) {
    if num.is_empty() || den.len() < 2 {
        panic!("Invalid transfer function.");
    }
    
    // Normalize if denominator's leading coefficient is not 1
    let (num, den) = if den[0] != 1.0 {
        normalize_tf(num, den)
    } else {
        (num.to_vec(), den.to_vec())
    };
    
    let n = den.len() - 1;
    
    // Initialize A Matrix (N x N)
    let mut a = DMatrix::zeros(n, n);
    
    // Fill A Matrix based on denominator coefficients
    for col in 0..n {
        if col > 0 {
            a[(col - 1, col)] = 1.0; // Subdiagonal (shift matrix)
        }
        a[(n - 1, col)] = -den[n - col]; // Last row
    }
    
    // Initialize B Vector (N x 1)
    let mut b = DMatrix::zeros(n, 1);
    b[(n - 1, 0)] = num[0];
    
    // Initialize C Matrix (1 x N)
    let mut c = DMatrix::zeros(1, n);
    for i in 0..n {
        c[(0, i)] = *num.get(i).unwrap_or(&0.0);
    }
    
    // Initialize D Scalar
    let d = 0.0;
    
    (a, b, c, d)
}

fn normalize_tf(num: &[f64], den: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let leading = den[0];
    let num: Vec<f64> = num.iter().map(|&e| e / leading).collect();
    let den: Vec<f64> = den.iter().map(|&e| e / leading).collect();
    (num, den)
}