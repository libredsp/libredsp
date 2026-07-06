use libredsp::signal::Signal;

fn main() {
    let signal = Signal::new(vec![1.0, 0.0, 0.0, 0.0]);
    println!("Signal: {:?}", signal.clone().to_vec());
    
    // Apply FFT
    let spectrum = signal.fft();
    println!("Result of FFT: {:?}", spectrum);
}