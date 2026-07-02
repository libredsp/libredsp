use std::f64::consts::PI;
use crate::signal::Signal;

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