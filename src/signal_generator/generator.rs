use std::{f64::consts::PI};
use crate::signal::Signal;

pub fn get_sine_signal(n: usize, amplitude: f64, frequency: f64, phase: f64) -> Signal {
    let mut res: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / n as f64;
        res.push(amplitude * (2.0 * PI * frequency * t + phase).sin());
    }
    Signal::new(res)
}


pub fn get_pulse_train_signal(n: usize, amplitude: f64, frequency: f64, duty_cycle: f64) -> Signal {
    let duty = duty_cycle.clamp(0.0, 1.0);
    let mut res: Vec<f64> = Vec::with_capacity(n);
    
    for i in 0..n {
        let normalized_time = i as f64 / n as f64;
        // Position within the cycle (0 to 1)
        let cycle_position = (frequency * normalized_time) % 1.0;
        
        if cycle_position < duty {
            res.push(amplitude);
        } else {
            res.push(0.0);  // Zero between pulses
        }
    }
    
    Signal::new(res)
}

pub fn get_white_noise_signal(n: usize, standard_deviation: f64, mean: f64) -> Signal {
    let mut res = Vec::with_capacity(n);

    for _ in 0..n {
        let u1: f64 = rand::random_range(0.0..1.0);
        let u2: f64 = rand::random_range(0.0..1.0);

        let z = (-2.0 * u1.ln()).sqrt()
            * (2.0 * std::f64::consts::PI * u2).cos();

        res.push(mean + standard_deviation * z);
    }

    Signal::new(res)
}