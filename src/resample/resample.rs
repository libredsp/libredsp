use crate::resample::{decimate, interpolate};
use crate::signal::Signal;
use crate::window::*;

/* NOT efficient. TODO impelement the resample function via polyphase filter */
pub fn resample(
    signal: &Signal,
    up_scale: usize,
    down_scale: usize,
    window_type: WindowType,
) -> Signal {
    let interpolated = interpolate(signal, up_scale, window_type);
    decimate(&interpolated, down_scale, window_type)
}
