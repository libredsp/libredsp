use std::ops::{Add, Index, IndexMut, Mul};

use crate::TransferFunction;

#[derive(Debug, Clone)]
pub struct Signal {
    pub(crate) data: Vec<f64>,
}

impl Signal {
    pub fn new(data: Vec<f64>) -> Self {
        Signal { data }
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
}

impl FromIterator<f64> for Signal {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Signal::new(iter.into_iter().collect())
    }
}

// Element-wise addition
impl<'a, 'b> Add<&'b Signal> for &'a Signal {
    type Output = Signal;

    fn add(self, other: &'b Signal) -> Signal {
        assert_eq!(self.len(), other.len());

        let result = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(&x, &y)| x + y)
            .collect();

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
