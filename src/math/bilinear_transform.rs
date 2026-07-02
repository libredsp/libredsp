/*
    Bilinear transform is simply a one-to-one map between the (s)-plane and the (z)-plane.

    Assume you have a system defined by a rational transfer function (H(s)).
    This system corresponds to a linear constant-coefficient differential equation.

    Convert that system into its corresponding differential equation,
    and then use the trapezoidal method for integration to obtain a difference equation.

    Applying the (z)-transform to this difference equation results in replacing (s) (in the original (H(s))) by

    \frac{2}{T}\left(\frac{1-z^{-1}}{1+z^{-1}}\right)

    **********************************************************************************************************

    Now, the algorithm does the following:

    Assume you have transfer function \(H(s)\) defined as:

    \[
    H(s)=
    \frac{
        b_0+b_1s+b_2s^2+\cdots+b_Ms^M
    }{
        a_0+a_1s+a_2s^2+\cdots+a_Ns^N
    }
    \]

    Define

    \[
    L=\max(M,N)
    \]

    The bilinear transform replaces

    \[
    s \rightarrow K\frac{1-z^{-1}}{1+z^{-1}}
    \]

    where

    \[
    K=\frac{2}{T}
    \]

    Substituting into \(H(s)\), we get:

    \[
    H(z)=
    \frac{
        \sum_{i=0}^{M}
        b_i
        \left(
        K\frac{1-z^{-1}}{1+z^{-1}}
        \right)^i
    }{
        \sum_{i=0}^{N}
        a_i
        \left(
        K\frac{1-z^{-1}}{1+z^{-1}}
        \right)^i
    }
    \]

    Expanding each term:

    \[
    H(z)=
    \frac{
        \sum_{i=0}^{M}
        b_i
        K^i
        \frac{(1-z^{-1})^i}{(1+z^{-1})^i}
    }{
        \sum_{i=0}^{N}
        a_i
        K^i
        \frac{(1-z^{-1})^i}{(1+z^{-1})^i}
    }
    \]

    Next, multiply numerator and denominator by

    \[
    (1+z^{-1})^L
    \]

    to eliminate fractional terms:

    \[
    H(z)=
    \frac{
        \sum_{i=0}^{M}
        b_i
        K^i
        (1-z^{-1})^i
        (1+z^{-1})^{L-i}
    }{
        \sum_{i=0}^{N}
        a_i
        K^i
        (1-z^{-1})^i
        (1+z^{-1})^{L-i}
    }
    \]

    Hence, for each coefficient we obtain the term

    \[
    cK^i
    (1-z^{-1})^i
    (1+z^{-1})^{L-i}
    \]


*/

use crate::types::TransferFunction;
use std::cmp::max;
use crate::math::convolve;
use crate::utils::vector_ops;

fn construct_z_plus_one_or_z_minus_one_polynomial(n: usize, i: usize) -> Vec<f64> {
    if n == 0 {
        vec![1.0];
    }

    let mut polynomial_coeffs: Vec<Vec<f64>> = Vec::new();

    for _ in 0..i {
        polynomial_coeffs.push(vec![1.0, -1.0]);
    }

    for _ in 0..(n - i) {
        polynomial_coeffs.push(vec![1.0, 1.0]);
    }

    let mut res = polynomial_coeffs[0].clone();

    for i in 1..polynomial_coeffs.len() {
        res = convolve::convolve(
            &res,
            &polynomial_coeffs[i],
        );
    }
    res
}

pub fn bilinear_transform(tf: TransferFunction) -> TransferFunction {
    let n = max(tf.num.len(), tf.den.len()) - 1;

    let k: f64 = 2.0;

    // Building the denominator
    let mut tmp_num: Vec<Vec<f64>> = Vec::new();

    for (i, c) in tf.num.iter().enumerate() {

        let poly =
            construct_z_plus_one_or_z_minus_one_polynomial(n, i);

        let scaled =
            vector_ops::a_value_times_elements_of_array(
                *c * k.powi(i as i32),
                &poly,
            );

        tmp_num.push(scaled);
    }

    let mut res_num =
        tmp_num.first().cloned().unwrap_or_default();

    for i in 1..tmp_num.len() {
        res_num =
            vector_ops::add_vectors(
                &res_num,
                &tmp_num[i],
            );
    }

    // Building the denominator
    let mut tmp_den: Vec<Vec<f64>> = Vec::new();

    for (i, c) in tf.den.iter().enumerate() {

        let poly =
            construct_z_plus_one_or_z_minus_one_polynomial(n, i);

        let scaled =
            vector_ops::a_value_times_elements_of_array(
                *c * k.powi(i as i32),
                &poly,
            );

        tmp_den.push(scaled);
    }

    let mut res_den =
        tmp_den.first().cloned().unwrap_or_default();

    for i in 1..tmp_den.len() {
        res_den =
            vector_ops::add_vectors(
                &res_den,
                &tmp_den[i],
            );
    }

    // Normalize so that the transfer fucntion would become
    // H(z)=\frac{b_0 + b_1z^{-1} + ... + b_Lz^{-L}}{1 + a_1z^{-1} + ... + a_Lz^{-L}}
    if res_den.is_empty() {
        panic!("Denominator array is empty");
    }

    let first = res_den[0];

    for x in &mut res_den {
        *x /= first;
    }

    for x in &mut res_num {
        *x /= first;
    }

    TransferFunction {
        num: res_num,
        den: res_den,
    }
    
}
