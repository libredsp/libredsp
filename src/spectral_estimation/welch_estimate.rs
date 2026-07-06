use crate::filter_design::window::*;
use crate::types::*;
use crate::signal::Signal;

impl Signal {
    pub fn welch_estimate(
        &self,
        window_type: WindowType,
        segment_size: usize,
        shift_size: usize
    ) -> Vec<f64> {
        let number_of_segments = 1 + (self.data.len() - segment_size) / shift_size;
        
        // For non-rectangular window, the window should be normalized
        let mut w = get_window(window_type, segment_size);
        let mut sum = 0.0;

        for i in 0..w.len() {
            sum += w[i] * w[i];
        }

        let window_normalization_factor = 1.0 / (sum / segment_size as f64).sqrt();

        for i in 0..w.len() {
            w[i] *= window_normalization_factor;
        }

        // Take one half. For real-valued signals one half is the complex conjugate of the other half.
        let mut res = vec![0.0; segment_size / 2 + 1];

        for k in 0..number_of_segments {
            let start = k * shift_size;
            let mut seg = self.slice(start, start + segment_size);
            seg = seg * &w;
            let spectrum = seg.fft();
            for m in 0..=segment_size / 2 {
                res[m] += (spectrum[m].re * spectrum[m].re + spectrum[m].im * spectrum[m].im) 
                            / segment_size as f64;
            }
        }

        for m in 0..res.len() {
            res[m] /= number_of_segments as f64;
        }

        res
    }
}