use libredsp::filter_design::statistical::{LMS, wiener};
use libredsp::signal::Signal;

#[test]
fn recovers_known_two_tap_fir_system() {
    let x = Signal::new(vec![1.0, 0.0, 1.0, 0.0]);
    // d[n] = x[n] + 0.5x[n-1], with x[-1] = 0
    let d = Signal::new(vec![1.0, 0.5, 1.0, 0.5]);
    let w = wiener(&x, &d, 2);

    // Wiener solution recovers the true filter because x was a whitenoise.
    assert!((w[0] - 1.0).abs() < 1e-10, "w[0] = {}", w[0]);
    assert!((w[1] - 0.5).abs() < 1e-10, "w[1] = {}", w[1]);
}

#[test]
fn lms_check_against_wiener() {
    let x = Signal::new(vec![1.0, 0.0, 1.0, 0.0]);
    let d = Signal::new(vec![1.0, 0.5, 1.0, 0.5]);

    let w_wiener = wiener(&x, &d, 2);

    let mut lms = LMS::new(2, 0.05);
    for _ in 0..1000 {
        for n in 0..x.len() {
            println!("{:?}", lms.coefficients());
            lms.update(x[n], d[n]);
        }
    }

    let w = lms.coefficients();
    // compare against Wiener, with a realistic tolerance
    assert!((w[0] - w_wiener[0]).abs() < 0.05, "w[0] = {}", w[0]);
    assert!((w[1] - w_wiener[1]).abs() < 0.05, "w[1] = {}", w[1]);
}
