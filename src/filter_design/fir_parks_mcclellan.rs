use crate::filter_design::linear_phase_cosine_to_impulse_response;
use crate::math::piecewiselinear_interpolation::PiecewiseLinearInterpolator;
use crate::types::LinearPhaseFilterType;
use crate::types::TransferFunction;

use nalgebra::DMatrix;
/*
        Functions of the following form (in the frequency domain):

            A(w) = Q(w) * sum_{k=0}^{n} c_k cos(kw)

        where Q(w) is one of four functions,

            Q(w) = 1,
            Q(w) = cos(w/2),
            Q(w) = sin(w),
            Q(w) = sin(w/2),

        yield linear-phase FIR filters.

        The objective of the Remez exchange algorithm is to find suitable coefficients c_k
        such that:

            max_{w} |E(w)|

        is minimized, where:

            E(w) = W(w)(D(w) - A(w))

        It can be shown that A(w) maps one-to-one to a polynomial on an interval. As a
        consequence, minimizing max_{w} |E(w)| is equivalent to finding a polynomial that
        minimizes that objetcive function (instead of a sum of cosines).

        There is a central theorem in polynomial approximation called the Alternation
        Theorem, which states that the polynomial of degree M, minimizing max_{w} |E(w)|
        at least at M + 2 alternating extremal points where the weighted error reaches
        *the same magnitude* and *alternates in sign*. These are the maximum absolute
        values of the weighted error over the frequency range of interest.

        The Remez exchange algorithm is used to find the optimal polynomial A(w) that minimizes
        the maximum E(w). This problem is also known as the 'minimax' problem in approx.
        theroy.
        At the beginning we do a few initializations. First, we initialize a vector named
        'dense_grid', which is simply a dense set of discrete frequencies on which we
        run the algorithm (because we cannot possibly evaluate every frequency in a continuous
        interval). We also initialize 'extrema_frequencies_indices', which holds indices into
        the 'dense_grid' vector corresponding to the current estimate of the extremal frequencies.
        Initially, these indices are chosen to be uniformly spaced over the 'dense_grid'.

        As the first step of the algorithm, we substitute those extremal frequencies
        (obtained via 'extrema_frequencies_indices') into the following expression:

            W(w_i)[A(w_i) - D(w_i)] = (-1)^i delta

        where delta is also an unknown, and i is the index of the extremal frequency
        (starting from 0). The above expression yields an N x N linear system of the
        form
           _                                                       _    _     _     _       _
          |  1  cos(w0)  cos(2w0)  ...  cos(mw0)   1/W(w0)          |  | d_0   |   |  D(w0)  |
          |  1  cos(w1)  cos(2w1)  ...  cos(mw1)  -1/W(w1)          |  | d_1   |   |  D(w1)  |
          |  .                                                      |  | .     | = |         |
          |  .                                                      |  | .     |   |         |
          |  .                                                      |  | d_n   |   |         |
          |_ 1  cos(wn)  cos(2wn)  ...  cos(mwn)  (-1)^n/W(wn)     _|  |_delta_|   |_  D(wn)_|

        or, in matrix notation, simply

            Ax = b.

        We then solve this system, obtaining coefficients for which the current
        extremal frequencies all have weighted error equal to +/-delta. However, this
        does not necessarily mean we have found the optimum solution, since another
        frequency in the 'dense_grid' may have a larger absolute error.

        Because of this, we sweep the entire 'dense_grid' and compute the weighted error
        at every discrete frequency. If the maximum absolute error we find is equal to
        delta (within a tolerance), then the algorithm has converged and we have found
        the minimax solution. Otherwise, we replace 'extrema_frequencies_indices' with
        the indices of the new extremal frequencies, those where the error has the largest
        absolute value and alternates in sign (for example, 0.3, -0.3, 0.3, ...).
        We then repeat the process until the algorithm converges.

        Note: To understand why we choose the extremal frequencies in each iteration
        of the algorithm, we must show that e_{k+1} < e_k, where e_i denotes the
        maximum weighted error between the approximation and the desired function
        in iteration 'i' of the algorithm. For a proof of this, see "Iske, Armin.
        Approximation theory and algorithms for data analysis."

        For a general reference on this algorithm, see the original paper,
        "Parks, T., and James McClellan. Chebyshev approximation for nonrecursive
        digital filters with linear phase."
*/

pub fn parks_mcclellan_filter_design(
    n: usize,
    desired_response: impl Fn(f64) -> f64,
    weighting: impl Fn(f64) -> f64,
    linear_phase_filter_type: LinearPhaseFilterType,
) -> TransferFunction {
    /*  ============================================================================================================
        ==============================================  Step I  ====================================================
        ============================================================================================================
        Initializations...
    */
    let (q, cosine_sum_upper_limit) =
        get_leading_fir_linear_phase_coef_and_cosine_sum_upper_limit(linear_phase_filter_type, n);

    /* Because of the alternation theorem, A(w) should be a sum of cosines. So, we modify other terms. */

    /* initialization step */
    let r = cosine_sum_upper_limit + 2;

    /* - discretizing the frequency axis */
    let dense_size = 40 * n;
    let dense_grid: Vec<f64> = (0..dense_size)
        .map(|i| (i as f64 / (dense_size - 1) as f64) * std::f64::consts::PI)
        .collect();

    /* Keep the points where W(w) != 0 */
    let frequency_grid: Vec<f64> = dense_grid
        .into_iter()
        .filter(|&w| weighting(w) > 0.0 /* Needed becuase if weighting(w) = 0 remains, later, we divide delta by weighting(w) which is division by zero */
                             &&
                     q(w).abs() > 1e-12)
        .collect();
    /* Uniform */
    let mut extrema_frequencies_indices: Vec<usize> = (0..r)
        .map(|i| (i * (frequency_grid.len() - 1)) / (r - 1))
        .collect();

    /* Precompute D(w) and W(w) once over the whole grid */
    let mut d_grid: Vec<f64> = Vec::with_capacity(frequency_grid.len());
    let mut w_grid: Vec<f64> = Vec::with_capacity(frequency_grid.len());
    for &w in &frequency_grid {
        d_grid.push(desired_response(w));
        w_grid.push(weighting(w));
    }

    let mut a_coeffs = vec![0.0; cosine_sum_upper_limit + 1];
    for _iteration in 0..10 {
        let mut omega: Vec<f64> = Vec::with_capacity(r);
        let mut w_grid_subset: Vec<f64> = Vec::with_capacity(r);

        for i in 0..r {
            omega.push(frequency_grid[extrema_frequencies_indices[i]]);
            w_grid_subset.push(w_grid[extrema_frequencies_indices[i]]);
        }

        let cols = cosine_sum_upper_limit + 2;
        let mut a_data: Vec<Vec<f64>> = Vec::new();
        let mut b_data: Vec<f64> = Vec::with_capacity(r);

        for k in 0..=cosine_sum_upper_limit {
            let mut col: Vec<f64> = Vec::with_capacity(r);
            for i in 0..r {
                col.push(q(omega[i]) * (k as f64 * omega[i]).cos());
            }
            a_data.push(col);
        }

        let mut last_col_of_a: Vec<f64> = Vec::with_capacity(r);
        for i in 0..r {
            b_data.push(d_grid[extrema_frequencies_indices[i]]);

            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            last_col_of_a.push(sign / w_grid_subset[i]);
        }

        a_data.push(last_col_of_a);

        let flat_a_data: Vec<f64> = a_data.into_iter().flatten().collect();

        let a_matrix = DMatrix::from_column_slice(r, cols, &flat_a_data);
        let b_vector = DMatrix::from_column_slice(r, 1, &b_data);

        let solution = a_matrix
            .lu()
            .solve(&b_vector)
            .expect("Remez system should be solvable (matrix should be non-singular)");

        /*  ============================================================================================================
            ==============================================  Step II ====================================================
            ============================================================================================================
            Here, we sweep the frequency grid and find the maximum errors between the desired frequency and the filter
            via the coefficients we found achieve...
        */

        for k in 0..=cosine_sum_upper_limit {
            a_coeffs[k] = solution[k];
        }

        /* Sweep the grid and compute weighted error E(w) */
        let mut error: Vec<f64> = Vec::with_capacity(frequency_grid.len());
        for i in 0..frequency_grid.len() {
            let a_w = get_filters_output_in_the_frequency_domain(&frequency_grid[i], &a_coeffs, q);
            error.push(w_grid[i] * (d_grid[i] - a_w));
        }

        /*  ============================================================================================================
            ==============================================  Step III ===================================================
            ============================================================================================================
            Update extrema frequencies...

        */

        /* Find the points that are 'max' or 'min' relative to their neighborhood, i.e., local min and max
           This is needed because let's say we are at a peak, then multiple points near the local max have large values.
           We should not consider all of them as extremas.
        */

        let candidate_indices = get_extrema_indices(&error);
        let new_reference_set = update_reference_set(candidate_indices, &error, r);

        /* Convergence test... */
        assert_eq!(new_reference_set.len(), r);

        if check_for_convergence(&extrema_frequencies_indices, &new_reference_set) {
            break;
        } else {
            extrema_frequencies_indices = new_reference_set;
        }
    }
    let h = linear_phase_cosine_to_impulse_response(&a_coeffs, linear_phase_filter_type);

    TransferFunction {
        num: h,
        den: vec![1.0],
    }
}

fn get_leading_fir_linear_phase_coef_and_cosine_sum_upper_limit(
    linear_phase_filter_type: LinearPhaseFilterType,
    n: usize,
) -> (fn(f64) -> f64, usize) {
    match linear_phase_filter_type {
        LinearPhaseFilterType::I => {
            assert!(n % 2 == 1);
            let half_length = (n - 1) / 2;
            (q_type_i, half_length)
        }
        LinearPhaseFilterType::II => {
            assert!(n % 2 == 0 && n >= 2);
            let half_length = n / 2;
            (q_type_ii, half_length - 1)
        }
        LinearPhaseFilterType::III => {
            assert!(n % 2 == 1 && n >= 3);
            let half_length = (n - 1) / 2;
            (q_type_iii, half_length - 1)
        }
        LinearPhaseFilterType::IV => {
            assert!(n % 2 == 0 && n >= 2);
            let half_length = n / 2;
            (q_type_iv, half_length - 1)
        }
    }
}

/* The following functions that multplies the sum of conine terms for different types of linear phase filters. */
fn q_type_i(_w: f64) -> f64 {
    1.0
}
fn q_type_ii(w: f64) -> f64 {
    (w / 2.0).cos() /* or, cos(\pi f) */
}
fn q_type_iii(w: f64) -> f64 {
    w.sin()
}
fn q_type_iv(w: f64) -> f64 {
    (w / 2.0).sin()
}

fn get_filters_output_in_the_frequency_domain(
    omega: &f64,
    a_coeffs: &[f64],
    q: fn(f64) -> f64,
) -> f64 {
    let mut res: f64 = 0.0;
    for k in 0..a_coeffs.len() {
        res += a_coeffs[k] * (k as f64 * omega).cos();
    }
    q(*omega) * res /* Multiply by a particular q(w), depending on the linear phase filter type. */
}

fn get_extrema_indices(error: &Vec<f64>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    for i in 0..error.len() {
        /* boundry indices are always exterma. Push to result */
        if i == 0 || i == error.len() - 1 {
            res.push(i);
        } else if (error[i] >= error[i - 1] && error[i] >= error[i + 1])
            || (error[i] <= error[i - 1] && error[i] <= error[i + 1])
        {
            res.push(i);
        }
    }
    res
}


fn check_for_convergence(current_reference_set: &[usize], new_reference_set: &[usize]) -> bool {
    current_reference_set == new_reference_set
}

fn update_reference_set(mut candidates: Vec<usize>, error: &[f64], r: usize) -> Vec<usize> {
    if candidates.len() > r {
        candidates.sort_by(|&a, &b| error[b].abs().partial_cmp(&error[a].abs()).unwrap());
        candidates.truncate(r);
        candidates.sort_unstable();
    }
    candidates
}

pub fn parks_mcclellan_filter_design_by_nodes(
    n: usize,
    desired_response_nodes: Vec<(f64, f64)>,
    weighting_nodes: Vec<(f64, f64)>,
    linear_phase_filter_type: LinearPhaseFilterType,
) -> TransferFunction {
    let desired_response_interpolator = PiecewiseLinearInterpolator::new(desired_response_nodes);
    let weighting_interpolator = PiecewiseLinearInterpolator::new(weighting_nodes);

    parks_mcclellan_filter_design(
        n,
        |f: f64| desired_response_interpolator.evaluate(f),
        |f: f64| weighting_interpolator.evaluate(f),
        linear_phase_filter_type,
    )
}
