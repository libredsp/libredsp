use libredsp::simulator::node_types::{Generator};
use libredsp::types::GeneratorType;
use libredsp::signal::Signal;
use libredsp::simulator::Node;

fn main() {
    let n = 64;
    let freq = 5.0;
    let amplitude = 1.0;
    
    // Create generators and set their IDs
    let mut sine_gen = Generator::new(GeneratorType::Sine {
        n,
        amplitude,
        frequency: freq,
        phase: 0.0,
    });
    sine_gen.set_id(0);
    
    let mut noise_gen = Generator::new(GeneratorType::WhiteNoise {
        n,
        standard_deviation: 2.0,
        mean: 0.0,
    });
    noise_gen.set_id(1);
    
    // Extract and combine signals
    let mut samples = Vec::with_capacity(n);
    for _ in 0..n {
        let sine_val = sine_gen.execute(&[]).unwrap().output;
        let noise_val = noise_gen.execute(&[]).unwrap().output;
        samples.push(sine_val + noise_val);
    }
    
    let signal = Signal::new(samples);
    let psd = signal.welch_estimate(libredsp::types::WindowType::Rectangular, 64, 5);

    // We can see the non-zero peak at bin 5
    println!("{:?}", psd);
}