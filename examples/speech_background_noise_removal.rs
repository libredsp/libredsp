use std::path::Path;

use libredsp::speech::stft_background_noise_removal;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 7 {
        eprintln!(
            "usage: speech_background_noise_removal <input.wav> <output.wav> <noise_start> <noise_end> <frame_len_ms> <alpha>"
        );
        return;
    }

    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);

    let noise_start: f64 = args[3].parse().unwrap();
    let noise_end: f64 = args[4].parse().unwrap();
    let frame_len_ms: f64 = args[5].parse().unwrap();
    let alpha: f64 = args[6].parse().unwrap();

    stft_background_noise_removal::stft_background_noise_removal(
        input,
        output,
        noise_start,
        noise_end,
        frame_len_ms,
        alpha,
    )
    .unwrap();
}
