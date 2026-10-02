use libredsp::speech::wav::read_wav;
use libredsp::speech::{LogisticRegression, VadConfig, detect_speech};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: detect <model.txt> <recording.wav>");
        return;
    }

    let model = LogisticRegression::load(&args[1]).unwrap();
    let (signal, sample_rate) = read_wav(&args[2]).unwrap();

    let segments = detect_speech(&model, &signal, sample_rate, &VadConfig::default());

    for s in segments {
        let label = if s.is_speech { "speech" } else { "non-speech" };
        println!("{:7.2} {:7.2}  {label}", s.start, s.end);
    }
}
