pub fn bessel_function_zero_order(x: f64) -> f64 {
    // Handle the edge case to prevent infinite loop
    if x == 0.0 {
        return 1.0;
    }

    let mut term = 1.0;
    let mut sum = 1.0;
    let mut k = 1;

    loop {
        term *= (x / (2.0 * k as f64)).powi(2);
        
        sum += term;

        if (term / sum).abs() < 1e-16 {
            break;
        }

        k += 1;
    }

    sum
}