use crate::types::*;
use crate::filter_design::analog_filter_design;
use crate::math::bilinear_transform;
use num_complex::Complex;
use crate::math::convolve::convolve;

pub fn analog_to_digital_transform_iir_filter_design(
    method: AnalogToDigitalTransformationDesignMethod,
    filter_type: FilterType,
    n: usize,
    chebyshev_epsilon_factor: f64
    ) -> TransferFunction {
        // Steps:
        // 1. Find cutoff frequency of analog filter via: Omega = 2*tan(w/2).
        // 2. Deisng an analog filter with the determined cutoff freq.
        // 3. Use bilinear transform to convert the analaog filter to digital filter.

        // 1.
        let omega_c: f64;
        match filter_type {
            FilterType::Lowpass { w } => {
                omega_c = 2.0 * (w / 2.0).tan();
            }

            FilterType::Highpass { w } => {
                omega_c = 2.0 * (w / 2.0).tan();
            }

            FilterType::Bandpass { .. } => {
                todo!("Bandpass not implemented yet")
            }

            FilterType::Bandstop { .. } => {
                todo!("Bandstop not implemented yet")
            }
        };
        // 2.
        let poles: Vec<Complex<f64>>;
        match method {
            AnalogToDigitalTransformationDesignMethod::Butterworth => {
                poles = analog_filter_design::get_causal_butterworth_poles(n, omega_c);
            }

            AnalogToDigitalTransformationDesignMethod::Chebyshev => {
                poles = analog_filter_design::get_chebyshev_i_poles(n, omega_c, chebyshev_epsilon_factor);
            }

        }

        let h_of_s = h_of_s(&poles, omega_c, &filter_type);
        let h_of_z = bilinear_transform::bilinear_transform(h_of_s);
        
    h_of_z
}

pub fn h_of_s(
    poles: &[Complex<f64>],
    omega_c: f64,
    filter_type: &FilterType,
) -> TransferFunction {
    let mut tmp: Vec<Vec<Complex<f64>>> = Vec::new();

    for pole in poles {
        tmp.push(vec![
            Complex::new(1.0, 0.0),
            Complex::new(-pole.re, -pole.im),
        ]);
    }

    let mut conv_res = tmp[0].clone();

    for i in 1..tmp.len() {
        conv_res = convolve(&conv_res, &tmp[i]);
    }

    // Construct den and num
    let mut den = vec![1.0];
    for i in 1..conv_res.len() {
        den.push(conv_res[i].re);
    }

    // Push reverse ordering
    let num = vec![omega_c.powi(poles.len() as i32)];

    match filter_type {
        FilterType::Lowpass { .. } => build_tf(num, den),

        FilterType::Highpass { .. } => {
            transform_analog_lowpass_to_highpass(
                poles,
                omega_c,
            )
        }

        FilterType::Bandpass { .. } => {
            todo!("Bandpass not implemented")
        }

        FilterType::Bandstop { .. } => {
            todo!("Bandstop not implemented")
        }
    }
}

fn build_tf(mut num: Vec<f64>, mut den: Vec<f64>) -> TransferFunction {
    num.reverse();
    den.reverse();
    TransferFunction {
        num,
        den,
    }
}

fn transform_analog_lowpass_to_highpass(poles: &[Complex<f64>], omega_c: f64) -> TransferFunction {
    println!("{}", omega_c);
    let hp_poles: Vec<Complex<f64>> =
        poles.iter().map(|p| {
            let denom =
                p.re * p.re + p.im * p.im;
            Complex::new(
                (omega_c * omega_c * p.re) / denom,
                (-omega_c * omega_c * p.im) / denom,
            )
        }).collect();

    let tmp: Vec<Vec<Complex<f64>>> = hp_poles.iter().map(|p| {
            vec![
                Complex::new(1.0, 0.0),
                Complex::new(-p.re, -p.im),
            ]
        }).collect();

    if tmp.is_empty() {
        panic!("hp_poles array is empty");
    }

    let mut tmp2 = tmp[0].clone();

    for i in 1..tmp.len() {
        tmp2 = convolve(&tmp2, &tmp[i]);
    }

    let conv_res = tmp2;

    let mut den: Vec<f64> = vec![1.0];

    for i in 1..conv_res.len() {
        den.push(conv_res[i].re);
    }

    let mut num = vec![0.0; poles.len() + 1];

    num[0] = omega_c.powi(poles.len() as i32);

    let gain = num[0] / den[0];

    num = num
        .iter()
        .map(|c| c / gain)
        .collect();

    build_tf(num, den)
}