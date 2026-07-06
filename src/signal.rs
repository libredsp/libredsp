use std::ops::{Add, Mul, Index, IndexMut};

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
    
    pub fn to_vec(self) -> Vec<f64> {
        self.data
    }

    pub fn zero_pad(&mut self, n: usize) {
        for _ in 0..n {
            self.data.push(0.0);
        }
    }
    pub fn zero_pad_to_the_next_power_of_two(&mut self) {
        let the_next_power_of_two = self.len().next_power_of_two();
        for _ in self.len()..the_next_power_of_two {
            self.data.push(0.0);
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> Signal {
        Signal::new(self.data[start..end].to_vec())
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

impl Mul<&Signal> for Signal {
    type Output = Signal;
    
    fn mul(self, other: &Signal) -> Signal {
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

// Index and IndexMut impl. to access and modify Signal's elements
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