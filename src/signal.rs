use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Signal {
    pub(crate) data: Vec<f64>,
}

impl Signal {
    pub fn new(data: Vec<f64>) -> Self {
        Signal { data }
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    // pub fn is_empty(&self) -> bool {
    //     self.data.is_empty()
    // }
    
    // pub fn as_slice(&self) -> &[f64] {
    //     &self.data
    // }
    
    pub fn to_vec(self) -> Vec<f64> {
        self.data
    }

    pub fn zero_pad(&mut self, n: usize) {
        for _ in 0..n {
            self.data.push(0.0);
        }
    }
}

// Element-wise addition
impl Add for Signal {
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

// Element-wise multiplication
impl Mul for Signal {
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

// Scalar multiplication
impl Mul<f64> for Signal {
    type Output = Signal;
    
    fn mul(self, scalar: f64) -> Signal {
        let result: Vec<f64> = self.data.iter().map(|&x| x * scalar).collect();
        Signal::new(result)
    }
}
