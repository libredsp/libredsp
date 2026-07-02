use std::ops::{AddAssign, Mul};

pub fn convolve<T>(a: &[T], b: &[T]) -> Vec<T>
where T: Copy + Default + AddAssign + Mul<Output = T>,
{
    let m = a.len() + b.len() - 1;
    let mut res = vec![T::default(); m];
    for i in 0..a.len() {
        for j in 0..b.len() {
            res[i + j] += a[i] * b[j];
        }
    }

    res
}