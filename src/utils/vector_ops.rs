pub fn a_value_times_elements_of_array(k: f64, arr: &Vec<f64>) -> Vec<f64> {
    let mut res = Vec::with_capacity(arr.len());
    for i in 0..arr.len() {
        res.push(arr[i] * k);
    }
    res
}

pub fn add_vectors(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64> {

    assert!(
        v1.len() == v2.len(),
        "Vector sizes do not match"
    );

    let mut res = vec![0.0; v1.len()];

    for i in 0..v1.len() {
        res[i] = v1[i] + v2[i];
    }

    res
}
