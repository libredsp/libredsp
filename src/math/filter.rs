pub fn filter(
    signal: &[f64],
    num: &[f64],
    den: &[f64],
) -> Vec<f64> {
    let mut y_buffer = vec![0.0; den.len()];
    let mut result = Vec::with_capacity(signal.len());

    for i in 0..signal.len() {
        let mut x_term_sums = 0.0;

        for j in 0..num.len() {
            let x = if i >= j {
                signal[i - j]
            } else {
                0.0
            };

            x_term_sums += num[j] * x;
        }

        let mut y_term_sums = 0.0;

        for j in 1..den.len() {
            let y = if i >= j {
                y_buffer[j - 1]
            } else {
                0.0
            };

            y_term_sums += den[j] * y;
        }

        let output = x_term_sums - y_term_sums;

        for j in (1..y_buffer.len()).rev() {
            y_buffer[j] = y_buffer[j - 1];
        }

        y_buffer[0] = output;

        result.push(output);
    }

    result
}

// pub fn validate_filter_parameters(
//     filter_type: FilterType,
//     w1: f64,
//     w2: f64,
// ) -> Result<(), String> {
//     match filter_type {
//         FilterType::Lowpass | FilterType::Highpass => {
//             if w1 <= 0.0 || w1 >= std::f64::consts::PI {
//                 return Err("w1 must be between 0 and PI".to_string());
//             }
//         }
//         FilterType::Bandpass | FilterType::Bandstop => {
//             if w1 <= 0.0 || w2 <= w1 || w2 >= std::f64::consts::PI {
//                 return Err("Invalid frequency range for bandpass/bandstop filter".to_string());
//             }
//         }
//     }
//     Ok(())
// }