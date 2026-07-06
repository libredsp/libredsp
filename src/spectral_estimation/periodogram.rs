use crate::signal::Signal;

// Zero-pads the signal to the nearest power of 2, and computes the periodogram for it
impl Signal {
    pub fn periodogram(&self) -> Vec<f64> {
        let mut cloned_data = self.data.clone();

        for _ in cloned_data.len()..cloned_data.len().next_power_of_two() {
            cloned_data.push(0.0);
        }

        let zero_padded_signal = Signal::new(cloned_data);

        let spectrum = zero_padded_signal.fft();
        let fft_len = spectrum.len();

        let mut res: Vec<f64> = Vec::new();

        for i in 0..fft_len / 2 {
            res.push(
                spectrum[i].norm_sqr() / fft_len as f64
            );
        }

        res
    }
}