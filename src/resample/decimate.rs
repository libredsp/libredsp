use crate::filter_design::frequency_selective::windowing_method;
use crate::signal::Signal;
use crate::types::*;
use crate::window::*;
/*
 * Essentially, apply LPF, then keep every order-th element.
 */
pub fn decimate(signal: &Signal, order: usize, window_type: WindowType) -> Signal {
    let filter = windowing_method(
        order,
        window_type,
        FilterType::Lowpass {
            w: std::f64::consts::PI / order as f64,
        },
    );

    let filtered = signal.filter(&filter);

    filtered.iter().step_by(order).collect()
}
