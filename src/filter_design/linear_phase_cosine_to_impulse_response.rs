use crate::types::LinearPhaseFilterType;

pub fn linear_phase_cosine_to_impulse_response (
    a_coeffs: &[f64],
    linear_phase_filter_type: LinearPhaseFilterType,
) -> Vec<f64> {
    let a = |j: isize| -> f64 {
        if j < 0 || j as usize >= a_coeffs.len() {
            0.0
        } else {
            a_coeffs[j as usize]
        }
    };

    match linear_phase_filter_type {
        LinearPhaseFilterType::I => {
            let m = a_coeffs.len() - 1;
            let mut h = vec![0.0; 2 * m + 1];
            h[m] = a_coeffs[0];
            for k in 1..=m {
                h[m - k] = a_coeffs[k] * 0.5;
                h[m + k] = a_coeffs[k] * 0.5;
            }
            h
        }
        LinearPhaseFilterType::II => {
            /*
            Derivation:
            We have the coefficients a_k for a Type II filter, which has the form

                A(w) = cos(w/2) sum_{k=0}^{m-1} a_k cos(kw)

            It can be shown that the corresponding frequency response is the DTFT of a
            symmetric impulse response satisfying

                h[m-k] = h[m+k-1]

            From the trigonometric identity

                cos(a)cos(b) = 0.5[cos(a+b) + cos(a-b)]

            we obtain

                cos(w/2)cos(kw)
                = 0.5[cos((k+0.5)w) + cos((k-0.5)w)]

            Substituting into A(w),

                A(w)
                = 0.5 sum_{k=0}^{m-1}
                  a_k [cos((k+0.5)w) + cos((k-0.5)w)]

            Collecting like terms gives

                coefficient of cos(w/2)         = a_0 + 0.5a_1      := b_1
                coefficient of cos(3w/2)        = 0.5(a_1 + a_2)    := b_2
                coefficient of cos(5w/2)        = 0.5(a_2 + a_3)    := b_3
                    ...
                coefficient of cos((m-0.5)w)    = 0.5a_{m-1}         := b_m

            Thus,

                A(w) = sum_{k=1}^{m} b_k cos((k-0.5)w)

            On the other hand, the DTFT of a symmetric impulse response is

                H(w)
                = exp(-j(m-0.5)w)
                  sum_{k=1}^{m} 2h[m-k] cos((k-0.5)w)

            Since

                H(w) = exp(-j(m-0.5)w) A(w)

            comparing the cosine-series coefficients yields

                b_k = 2h[m-k]
            */
            let m = a_coeffs.len();
            let mut h = vec![0.0; 2 * m];
            for n in 0..m {
                let b = if n == 0 {
                    a(0) + 0.5 * a(1)
                } else if n == m - 1 {
                    0.5 * (a((m - 1) as isize))
                } else {
                    0.5 * (a(n as isize) + a(n as isize + 1))
                };

                let val = b / 2.0;
                h[m - 1 - n] = val;
                h[m + n] = val;
            }
            h
        }
        LinearPhaseFilterType::III => {
            /*
                Derivation:
                We have the coefficients a_k for a Type III filter, which has the form

                    A(w) = sin(w) sum_{k=0}^{m-1} a_k cos(kw)

                It can be shown that the corresponding frequency response is the DTFT of an
                antisymmetric impulse response satisfying

                    h[m-k] = -h[m+k]

                From the trigonometric identity

                    sin(a)cos(b) = 0.5[sin(a+b) + sin(a-b)]

                we obtain

                    sin(w)cos(kw)
                    = 0.5[sin((k+1)w) + sin((1-k)w)]

                Using

                    sin(-x) = -sin(x),

                this becomes

                    sin(w)cos(kw)
                    = 0.5[sin((k+1)w) - sin((k-1)w)].

                Substituting into A(w),

                    A(w)
                    = 0.5 sum_{k=0}^{m-1}
                      a_k [sin((k+1)w) - sin((k-1)w)]

                Collecting like terms gives

                    coefficient of sin(w)      = a_0 - 0.5a_2      := c_1
                    coefficient of sin(2w)     = 0.5(a_1 - a_3)    := c_2
                    coefficient of sin(3w)     = 0.5(a_2 - a_4)    := c_3
                        ...
                    coefficient of sin(mw)     = 0.5a_{m-1}        := c_m

                Thus,

                    A(w) = sum_{k=1}^{m} c_k sin(kw)

                On the other hand, the DTFT of an antisymmetric impulse response is

                    H(w) = exp(-jmw) sum_{k=1}^{m} 2h[m-k] sin(kw)

                Since

                    H(w) = exp(-jmw) A(w),

                comparing the sine-series coefficients yields

                    c_k = 2h[m-k]
            */
            let m = a_coeffs.len();
            let mut h = vec![0.0; 2 * m + 1];

            for n in 0..m {
                let c = if n == 0 {
                    a(0) - 0.5 * a(2)
                } else if n == m - 1 {
                    0.5 * a((m - 1) as isize)
                } else {
                    0.5 * (a(n as isize) - a(n as isize + 2))
                };

                let val = c / 2.0;
                h[m - 1 - n] = val;
                h[m + 1 + n] = -val;
            }

            h[m] = 0.0; /* Center, from h[n]=−h[N−1−n], we have h[m]=-h[m] => h[m]=0 */
            h
        }
        LinearPhaseFilterType::IV => {
            /*
                Derivation similar to Type III, except the leading coefficient is
                sin(w/2) instead of sin(w).

                This changes the sine basis from integer harmonics sin(kw) to
                half-integer harmonics sin((k-0.5)w).

                Collecting terms gives:

                    coefficient of sin(w/2)       = a_0 - 0.5a_1      := d_1
                    coefficient of sin(3w/2)      = 0.5(a_1 - a_2)    := d_2
                    coefficient of sin(5w/2)      = 0.5(a_2 - a_3)    := d_3
                    ...
                    coefficient of sin((m-0.5)w)  = 0.5a_{m-1}        := d_m

                Therefore,

                    A(w) = sum_{k=1}^{m} d_k sin((k-0.5)w)

                Comparing with the DTFT of an antisymmetric impulse response:

                    H(w) = exp(-j(m-0.5)w)
                           sum_{k=1}^{m} 2h[m-k] sin((k-0.5)w)

                gives:

                    d_k = 2h[m-k].
            */
            let m = a_coeffs.len();
            let mut h = vec![0.0; 2 * m];

            for n in 1..=m {
                let d = if n == 1 {
                    a(0) - 0.5 * a(1)
                } else if n == m {
                    0.5 * a((m - 1) as isize)
                } else {
                    0.5 * (a((n - 1) as isize) - a(n as isize))
                };

                h[m - n] = d / 2.0;
                h[m - 1 + n] = -d / 2.0;
            }
            h
        }
    }
}
