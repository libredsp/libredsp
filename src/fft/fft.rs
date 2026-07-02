use crate::signal::Signal;
use std::f64::consts::PI;
use num_complex::Complex;

type Complex64 = Complex<f64>;

impl Signal {
    pub fn fft(&self) -> Vec<Complex64> {
        let input: Vec<Complex64> = self.data.iter()
            .map(|&x| Complex64::new(x, 0.0))
            .collect();
        fft_recursive(&input)
    }

    pub fn ifft(spectrum: &[Complex64]) -> Signal {
        let n = spectrum.len();
        
        // Conjugate, compute FFT, conjugate again, and scale
        let conjugated: Vec<Complex64> = spectrum.iter().map(|x| x.conj()).collect();
        let mut result = fft_recursive(&conjugated);
        
        for x in &mut result {
            *x = x.conj() / n as f64;
        }
        
        // Convert back to Signal (real values)
        let real_data: Vec<f64> = result.iter().map(|x| x.re).collect();
        Signal::new(real_data)
    }
}

fn get_even_elements(input: &[Complex64]) -> Vec<Complex64> {
    let mut result = Vec::with_capacity(input.len() / 2);
    for i in (0..input.len()).step_by(2) {
        result.push(input[i]);
    }
    result
}

fn get_odd_elements(input: &[Complex64]) -> Vec<Complex64> {
    let mut result = Vec::with_capacity(input.len() / 2);
    for i in (1..input.len()).step_by(2) {
        result.push(input[i]);
    }
    result
}

fn fft_recursive(input: &[Complex64]) -> Vec<Complex64> {
    let n = input.len();
    assert!(n.is_power_of_two(), "Input length must be a power of 2, got {}", n);
    
    if n == 1 {
        return vec![input[0]];
    }
    
    // Split into even and odd elements
    let even_elements = get_even_elements(input);
    let odd_elements = get_odd_elements(input);
  
    // Recursive FFT on even and odd parts
    let even_fft = fft_recursive(&even_elements);
    let odd_fft = fft_recursive(&odd_elements);
    
    // Combine results
    let mut result = vec![Complex64::new(0.0, 0.0); n];
    
    for k in 0..n / 2 {
        let angle = -2.0 * PI * k as f64 / n as f64;
        let twiddle = Complex64::new(angle.cos(), angle.sin());
        
        let t = twiddle * odd_fft[k];
        result[k] = even_fft[k] + t;
        result[k + n / 2] = even_fft[k] - t;
    }
    
    result
}