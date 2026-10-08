use libredsp::speech::dataset::load_labeled_frames;
use libredsp::speech::{LogisticRegression, TrainConfig};
use std::fs;
use std::path::{Path, PathBuf};

/// Collect every .wav file in `dir` and its subfolders.
fn find_wavs(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            find_wavs(&path, found);
        } else if path.extension().and_then(|e| e.to_str()) == Some("wav") {
            found.push(path);
        }
    }
}

fn main() {
    let folder = std::env::args().nth(1).expect("usage: train <folder>");

    let mut train_x = Vec::new();
    let mut train_y = Vec::new();
    let mut val_x = Vec::new();
    let mut val_y = Vec::new();

    // Every name.wav needs a name.csv next to it
    let mut wav_paths = Vec::new();
    find_wavs(Path::new(&folder), &mut wav_paths);
    if wav_paths.is_empty() {
        panic!("no .wav files found in {folder}");
    }

    for wav_path in wav_paths {
        let csv_path = wav_path.with_extension("csv");

        let (x, y) =
            load_labeled_frames(wav_path.to_str().unwrap(), csv_path.to_str().unwrap()).unwrap();

        // First 80% of each recording trains, last 20% is held out
        let split = x.len() * 8 / 10;
        for (i, (features, label)) in x.into_iter().zip(y).enumerate() {
            if i < split {
                train_x.push(features);
                train_y.push(label);
            } else {
                val_x.push(features);
                val_y.push(label);
            }
        }
    }

    if train_x.is_empty() {
        panic!("no labeled frames found, check that each .wav has a matching .csv");
    }

    println!(
        "{} training frames, {} validation frames",
        train_x.len(),
        val_x.len()
    );

    let model = LogisticRegression::train(&train_x, &train_y, &TrainConfig::default()).unwrap();

    println!("train: {:?}", model.evaluate(&train_x, &train_y, 0.5));
    println!("held out: {:?}", model.evaluate(&val_x, &val_y, 0.5));

    /* Saving the model */
    model.save("model.txt").unwrap();
    println!("model saved to model.txt");

    // Check that loading gives back the same model
    let loaded = LogisticRegression::load("model.txt").unwrap();
    assert_eq!(model, loaded);
}
