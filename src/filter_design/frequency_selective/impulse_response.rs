use crate::signal::Signal;
use crate::types::FilterType;
use std::f64::consts::PI;

pub fn get_impulse_response(
    n: usize,
    filter: FilterType,
) -> Signal {
    match filter {
        FilterType::Lowpass { w } => {
            low_pass_impulse_response(w, n)            
        }

        FilterType::Highpass { w } => {
            bandpass_impulse_response(PI, w, n)
        }

        FilterType::Bandpass { w1, w2 } => {
            bandpass_impulse_response(w1, w2, n)
        }

        FilterType::Bandstop { w1, w2 } => {
            bandpass_impulse_response(PI, w2, n) + low_pass_impulse_response(w1, n)
        }
    }
}

pub fn low_pass_impulse_response(cutoff_freq: f64, m: usize) -> Signal {
    let mid = (m - 1) as f64 / 2.0;    
    let mut array = Vec::with_capacity(m);

    for i in 0..m {
        let n = i as f64 - mid;
        if n.abs() < f64::EPSILON {
            array.push(cutoff_freq / PI);
        } else {
            array.push((cutoff_freq * n).sin() / (PI * n));
        }
    }
    Signal::new(array)
}

pub fn bandpass_impulse_response(w1: f64, w2: f64, n: usize) -> Signal {
    let mut array = vec![0.0; n];
    let mid = n / 2;

    for i in 0..n {
        let k = i as isize - mid as isize;
        if k == 0 {
            array[i] = (w2 - w1) / PI;
        } else {
            array[i] = ((w2 * k as f64).sin() - (w1 * k as f64).sin()) / (PI * k as f64);
        }
    }
    Signal::new(array)
}
