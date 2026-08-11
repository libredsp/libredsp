use libredsp::filter_design::windowing_method;
use libredsp::types::{FilterType, WindowType};

#[test]
fn windowing_method_has_expected_form() {
    let res = windowing_method(10, WindowType::Han, FilterType::Lowpass { w: 2.0 });
    assert_eq!(res.num.len(), 10);
    assert_eq!(res.den, vec![1.0]);
}

#[test]
fn windowing_method_is_symmetric() {
    let res = windowing_method(15, WindowType::Bartlett, FilterType::Lowpass { w: 1.0 });
    let last_elem_num_idx = res.num.len() - 1;
    for i in 0..res.num.len() {
        assert!(
            (res.num[i] - res.num[last_elem_num_idx - i]).abs() < 1e-9,
            "Linear phase filter not symmetric!"
        );
    }
}
