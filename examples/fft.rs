use libredsp::signal::Signal;

fn main() {
    let mut signal = Signal::new(vec![1.0, 0.0, 0.0, 0.0]);
    println!("Signal: {:?}", signal.clone().to_vec());

    // Apply FFT
    signal.zero_pad(1000);
    let dtft = signal.fft();
    println!("The DTFT {:?}", dtft);
}
