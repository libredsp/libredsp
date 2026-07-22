use libredsp::signal_generator::generator::{ get_sine_signal, get_white_noise_signal };

fn main() {
        let n = 64;
        let freq = 5.0;
        let amplitude = 1.0;
        
        let signal = get_sine_signal(n, amplitude, freq, 0.0) 
                                + get_white_noise_signal(n,2.0,0.0);
        let psd = signal.welch_estimate(libredsp::types::WindowType::Rectangular, 64, 5);

        // We can see the non-zero peak at bin 5
        println!("{:?}", psd);
}