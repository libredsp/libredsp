use std::ops::{Add, Index, IndexMut, Mul};

use crate::TransferFunction;

#[derive(Debug, Clone)]
pub struct Signal {
    pub(crate) data: Vec<f64>,
}

impl Signal {
    /*
     * Some notes about the function below. AsRef is a trait that is kind of defined as:
     *
     * trait AsRef<T> {
     *    fn as_ref(&self) -> &T;
     *  }
     * When we write 'AsRef<[f64]>' we are saying accept parameter that has a type that implements:
     *
     *    fn as_ref(&self) -> &[f64];
     *
     * That is, it would lend a reference to a slice via its as_ref method.
     * Also, note that another notation for function signature is:
     *
     * pub fn new(data: impl AsRef<[f64]>) -> Self { ... }
     */
    pub fn new<T: AsRef<[f64]>>(data: T) -> Self {
        Signal {
            /* Dot-call rules: data has type T, but as_ref is declared as
             *     fn as_ref(&self)
             * so we either write (&data).as_ref() or just data.as_ref() and
             * let Rust figure it out.
             *
             * If a method is declared via any of the forms below:
             *     impl X for &T { fn f(self) {} }    // Self = &T, self: &T
             *     impl X for T  { fn f(&self) {} }   // Self = T,  self: &T
             *
             * Which are themselves, shorthand for:
             *     impl X for &T { fn f(self: &Self) {} }   // Self = &T, self: &T
             *     impl X for T  { fn f(self: &Self) {} }   // Self = T,  self: &T
             *
             * Then for
             *
             *     let v: T = ...;
             *     v.f();
             *
             * Rust invokes the first of these that matches the declared type of self:
             *     1. X::f(v)         // self: T,      receiver matched by value
             *     2. X::f(&v)        // self: &T,     compiler adds &
             *     3. X::f(&mut v)    // self: &mut T, compiler adds &mut
             * If none match, it calls Deref on the object and retries the above
             * on the result.
             *
             * Here AsRef::as_ref takes &self, so #2 matches and
             * X::f(&v) is what gets called.
             */
            data: data.as_ref().to_vec(),
        }
    }

    pub fn zeros(size: usize) -> Self {
        Signal::new(vec![0.0; size])
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.data.clone()
    }

    pub fn zero_pad(&mut self, n: usize) {
        self.data.extend(std::iter::repeat_n(0.0, n));
    }

    pub fn zero_pad_to_the_next_power_of_two(&mut self) {
        let next_power_of_two = self.len().next_power_of_two();
        self.zero_pad(next_power_of_two - self.len());
    }

    pub fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        self.data.iter().copied()
    }

    pub fn slice(&self, start: usize, end: usize) -> Signal {
        Signal::new(self.data[start..end].to_vec())
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.data
    }

    pub fn crosscorrelation(&self, other: &Signal, max_lag: usize) -> Vec<f64> {
        assert_eq!(self.len(), other.len());

        let n = self.len();
        let mut p = vec![0.0; max_lag + 1];

        for (lag, p_lag) in p.iter_mut().enumerate() {
            let mut sum = 0.0;

            for k in 0..n {
                let x = if k >= lag { self[k - lag] } else { 0.0 };
                sum += x * other[k];
            }

            *p_lag = sum / n as f64;
        }

        p
    }

    pub fn autocorrelation(&self, max_lag: usize) -> Vec<f64> {
        self.crosscorrelation(self, max_lag)
    }

    pub fn filter(&self, filter_coef: &TransferFunction) -> Signal {
        Signal::new(crate::math::filter::filter(
            &self.to_vec(),
            &filter_coef.num(),
            &filter_coef.den(),
        ))
    }

    /* functions that are used for speech processing */

    /* Root mean square */
    pub fn rms(&self) -> f64 {
        let mean_square = self.data.iter().map(|x| x * x).sum::<f64>() / self.data.len() as f64;
        mean_square.sqrt()
    }

    /* Zero crossing rate: measures the rate at which the two consecutive samples change sign.
     * For a sine signal, its a low value like 0.01
     * For a DC signal is zero.
     * For -1, 1, -1, 1, ... the value is 1.0
     */
    pub fn zero_crossing_rate(&self) -> f64 {
        if self.data.len() < 2 {
            return 0.0;
        }

        let crossings = self
            .data
            .windows(2)
            .filter(|window| {
                /* Only keep the consequtive pairs that change sign */
                (window[0] >= 0.0 && window[1] < 0.0) || (window[0] < 0.0 && window[1] >= 0.0)
            })
            .count(); /* count the windows that survived the filter. */

        /* divide by the total length (minus one) to get the 'rate' */
        crossings as f64 / (self.data.len() - 1) as f64
    }

    /* maginitude_spectrum computes FFT and find adds the magnitude of the freq bins. */
    pub fn magnitude_spectrum(&self) -> Vec<f64> {
        self.fft().iter().map(|x| x.norm()).collect()
    }
    /* spectral_centroid:  */
    pub fn spectral_centroid(&self, sample_rate: f64) -> f64 {
        spectral_centroid(&self.magnitude_spectrum(), sample_rate)
    }
}

pub fn spectral_centroid(magnitudes: &[f64], sample_rate: f64) -> f64 {
    let n = magnitudes.len();

    /* the formula for the 'center of a mass' for function is simply:
     * c = (sum over k, y_k * x_k) / (sum over k, x_k)
     * For "spectral centroid" we are computing the same thing except
     * x_k's are frequency bins and y_k's are the magnitudes.
     */

    /* For a singal sampled at f_s rate, we have have delta_f_in_hz = f_s/N */
    let delta_f_in_hz = sample_rate / n as f64;

    let mut weighted_sum = 0.0;
    let mut magnitude_sum = 0.0;
    for (k, &mag) in magnitudes.iter().enumerate() {
        let freq = k.min(n - k) as f64 * delta_f_in_hz; // |frequency| of bin k
        weighted_sum += freq * mag;
        magnitude_sum += mag;
    }

    if magnitude_sum == 0.0 {
        0.0
    } else {
        weighted_sum / magnitude_sum
    }
    /*
     * For a numerical example for the caluclation of the centroid consider a pure sinusoidal
     * signal in the analog domain to be a 100 Hz sine wave. We sample this singal with f_s = 1000 Hz.
     * If then apply the FFT on N=10 samples.
     * The bin resolution will then be: delta_freq_hz = f_s / N = 1000 / 10 = 100 Hz
     * Therefore, bin k=1 will have the positive magnitude of 5.
     * Also, bin k=9 (10-1) will also have the same positive magnitude of 5.
     *
     * Few notes. To understand why in the formula above, `freq[k] = min(k,N-k) * delta_f_in_hz`, because if
     * we dont use the min(...), we obtain:
     * k=1 => freq[1] = 1*100 = 1*100 = 100 Hz
     * k=9 => freq[9] = 9*100 = 9*100 = 900 Hz
     * Centroid = (100*5 + 900*5) / (5+5) = 500 Hz
     *
     * This is nonsense. We should have gotten 100 Hz.
     *
     * Now, if we use `freq[k] = min(k,N-k) * delta_f_in_hz`, we consider the negative frequency, which are
     * the artificat of using complex-values in FFT, to be a component of the positive frequency. Hence, we get:
     *
     * k=1 => min(1, 9)*100 = 1*100 = 100 Hz
     * k=9 => min(9, 1)*100 = 1*100 = 100 Hz
     * Centroid = (100*5 + 100*5) / (5+5) = 100 Hz.
     *
     * Which matches the orignial continous-time signal.
     *
     * Another note. A quick reminder/recap on the sampling theorem and why
     * `delta_freq_hz = f_s / N`.
     * Essentially, when we sample a signal with the frequency of f_s, that
     * corresponds to first creating replicas of the CTFT (continuous-time
     * Fourier transform), spaced by `2\pi f_s` first, then mapping that
     * (compressing/stretching) to the interval of 0 to 2\pi. The result of this mapping
     * is the DTFT of the discrete samples we obtain via sampling the continuous
     * signal.
     *
     * This mapping is why `omega = 2\pi f / f_s`: the continuous-time frequency
     * `2\pi f` is divided by the replica spacing `2\pi f_s`, which maps the
     * frequency axis to the normalized DTFT frequency axis.
     *
     * Then, when we compute the FFT of N samples, we are essentially taking N
     * equispaced samples from the DTFT.
     * This is exactly why the freq. resolution is computed as
     * `delta_freq_hz = f_s / N`, because a frequency spacing of f_s Hz corresponds
     * to `2\pi f_s` rad/s in the CTFT and that is mapped to `2\pi` in the DTFT.
     * Therefore, each of the N equispaced FFT samples corresponds to a frequency
     * spacing of `f_s / N` Hz.
     */
}

impl FromIterator<f64> for Signal {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Signal {
            data: iter.into_iter().collect(),
        }
    }
}

// Element-wise addition
impl<'a, 'b> Add<&'b Signal> for &'a Signal {
    type Output = Signal;

    fn add(self, other: &'b Signal) -> Signal {
        assert_eq!(self.len(), other.len());

        // let result = self
        //     .data
        //     .iter()
        //     .zip(&other.data)
        //     .map(|(&x, &y)| x + y)
        //     .collect();
        let result = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(&x, &y)| x + y)
            .collect::<Vec<f64>>();

        Signal::new(result)
    }
}

// Element-wise addition
impl Add<Signal> for Signal {
    type Output = Signal;

    fn add(self, other: Signal) -> Signal {
        let n = self.data.len().max(other.data.len());
        let mut result = Vec::with_capacity(n);

        for i in 0..self.data.len().min(other.data.len()) {
            result.push(self.data[i] + other.data[i]);
        }

        if self.data.len() > other.data.len() {
            result.extend_from_slice(&self.data[other.data.len()..]);
        } else {
            result.extend_from_slice(&other.data[self.data.len()..]);
        }

        Signal::new(result)
    }
}

impl<'a> Add<&'a Signal> for Signal {
    type Output = Signal;

    fn add(self, other: &'a Signal) -> Signal {
        let n = self.data.len().max(other.data.len());
        let mut result = Vec::with_capacity(n);

        for i in 0..self.data.len().min(other.data.len()) {
            result.push(self.data[i] + other.data[i]);
        }

        if self.data.len() > other.data.len() {
            result.extend_from_slice(&self.data[other.data.len()..]);
        } else {
            result.extend_from_slice(&other.data[self.data.len()..]);
        }

        Signal::new(result)
    }
}

// Element-wise multiplication
impl Mul<Signal> for Signal {
    type Output = Signal;

    fn mul(self, other: Signal) -> Signal {
        let n = self.data.len().max(other.data.len());
        let mut result = Vec::with_capacity(n);

        for i in 0..self.data.len().min(other.data.len()) {
            result.push(self.data[i] * other.data[i]);
        }

        if self.data.len() > other.data.len() {
            result.extend_from_slice(&self.data[other.data.len()..]);
        } else {
            result.extend_from_slice(&other.data[self.data.len()..]);
        }

        Signal::new(result)
    }
}

impl<'a> Mul<&'a Signal> for Signal {
    type Output = Signal;

    fn mul(self, other: &'a Signal) -> Signal {
        let n = self.data.len().max(other.data.len());
        let mut result = Vec::with_capacity(n);

        for i in 0..self.data.len().min(other.data.len()) {
            result.push(self.data[i] * other.data[i]);
        }

        if self.data.len() > other.data.len() {
            result.extend_from_slice(&self.data[other.data.len()..]);
        } else {
            result.extend_from_slice(&other.data[self.data.len()..]);
        }

        Signal::new(result)
    }
}

// Scalar multiplication
impl Mul<f64> for Signal {
    type Output = Signal;

    fn mul(self, scalar: f64) -> Signal {
        let result: Vec<f64> = self.data.iter().map(|&x| x * scalar).collect();

        Signal::new(result)
    }
}
// Index and IndexMut
impl Index<usize> for Signal {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Signal {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
