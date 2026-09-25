use crate::filter_design::frequency_selective::windowing_method;
use crate::signal::Signal;
use crate::types::*;
use crate::window::*;

/*
 * Essentially, insert zeros between samples, then apply an LPF.
 */
pub fn interpolate(signal: &Signal, order: usize, window_type: WindowType) -> Signal {
    let mut data = Vec::with_capacity(signal.len() * order);

    for sample in signal.iter() {
        data.push(sample);

        for _ in 1..order {
            data.push(0.0);
        }
    }

    let result = Signal::new(data);

    let filter = windowing_method(
        order,
        window_type,
        FilterType::Lowpass {
            w: std::f64::consts::PI / order as f64,
        },
    );

    result.filter(&filter)
}
