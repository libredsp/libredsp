use std::f64::consts::PI;
use crate::{signal::Signal, types::WindowType};
use crate::math::{bessel_function_zero_order};

pub fn get_window(window_type: WindowType, size: usize) -> Signal {
    match window_type {
        WindowType::Rectangular => {
            Signal::new(vec![1.0; size])
        }
        WindowType::Hamming => {
            hamming(size)
        }
        WindowType::Han => {
            han(size)
        }
        WindowType::Bartlett => {
            bartlett(size)
        }
        WindowType::Kaiser{min_stopband_attinuation, transition_width} => {
            kaiser(min_stopband_attinuation, transition_width)
        }
    }
}

pub fn kaiser(
    min_stopband_attenuation: f64, // in dB (e.g., 60.0)
    transition_width: f64,         // in radians 0 to 2pi
) -> Signal {

    let beta = if min_stopband_attenuation <= 21.0 {
        0.0
    } else if min_stopband_attenuation <= 50.0 {
        0.5842 * (min_stopband_attenuation - 21.0).powf(0.4)
            + 0.07886 * (min_stopband_attenuation - 21.0)
    } else {
        0.1102 * (min_stopband_attenuation - 8.7)
    };

    let delta_f = transition_width / (2.0 * PI); 
    let raw_length = (min_stopband_attenuation - 7.95) / (14.36 * delta_f) + 1.0;

    let mut n = raw_length.ceil() as usize;
    if n < 3 { n = 3; }

    let denominator = bessel_function_zero_order(beta);
    let mut w = vec![0.0; n];
    let m = (n - 1) as f64;

    for i in 0..n {
        let i_f64 = i as f64;
        let center: f64 = (2.0 * i_f64 / m) - 1.0;
        let inner_term_sqrt = 1.0 - center * center;
        let inner_term = if inner_term_sqrt < 0.0 { 0.0 } else { inner_term_sqrt.sqrt() };
        
        let bessel_arg = beta * inner_term;
        w[i] = bessel_function_zero_order(bessel_arg) / denominator;
    }

    Signal::new(w)
}

pub fn hamming(m: usize) -> Signal {
    let mut array = vec![0.0; m];
    for i in 0..m {
        array[i] = 0.54 - 0.46 * ((2.0 * PI * i as f64) / ((m - 1) as f64)).cos();
    }
    Signal::new(array)
}

pub fn bartlett(m: usize) -> Signal {
    let mut array = vec![0.0; m];
    for i in 0..m {
        array[i] = 1.0 - (2.0 * (i as f64 - (m - 1) as f64 / 2.0).abs()) / ((m - 1) as f64);
    }
    Signal::new(array)
}

pub fn han(m: usize) -> Signal {
    let mut array = vec![0.0; m];
    for i in 0..m {
        array[i] = 0.5 * (1.0 - ((2.0 * PI * i as f64) / ((m - 1) as f64)).cos());
    }
    Signal::new(array)
}