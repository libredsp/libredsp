use libredsp::signal::Signal;
use libredsp::types::TransferFunction;
fn main() {
    let signal = Signal::new(vec![1.0, 0.0, 0.0, 0.0]);
    println!("Signal: {:?}", signal.clone().to_vec());

    let filter_coef = TransferFunction::new(vec![1.0, 0.5], vec![1.0, 0.3]);
    let output = signal.filter(&filter_coef);
    println!("Output: {:?}", output);
}
