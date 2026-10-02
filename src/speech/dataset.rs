use std::error::Error;
use std::fs;

use super::features::extract_features;
use super::frame_params;
use super::wav::read_wav;

/// A labeled interval of a recording, in seconds.
pub struct Annotation {
    pub start: f64,
    pub end: f64,
    pub is_speech: bool,
}

/// Parse lines of `start,end,label` where label is `speech` or `nonspeech`.
/// Blank lines and a header line are skipped.
pub fn parse_annotations(text: &str) -> Result<Vec<Annotation>, Box<dyn Error>> {
    let mut annotations = Vec::new();

    for line in text.lines() {
        let parts: Vec<&str> = line.split(',').map(str::trim).collect();
        if parts.len() != 3 {
            continue;
        }
        // The header line ("start,end,label") fails to parse and is skipped
        let (Ok(start), Ok(end)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) else {
            continue;
        };
        let is_speech = match parts[2] {
            "speech" => true,
            "nonspeech" => false,
            other => return Err(format!("unknown label '{other}'").into()),
        };
        annotations.push(Annotation {
            start,
            end,
            is_speech,
        });
    }

    Ok(annotations)
}

/// Load one recording and its annotation file as (features, labels).
/// Frames that fall outside every annotation are left out.
pub fn load_labeled_frames(
    wav_path: &str,
    csv_path: &str,
) -> Result<(Vec<Vec<f64>>, Vec<bool>), Box<dyn Error>> {
    let (signal, sample_rate) = read_wav(wav_path)?;
    let annotations = parse_annotations(&fs::read_to_string(csv_path)?)?;
    let (frame_length, hop) = frame_params(sample_rate);

    let mut x = Vec::new();
    let mut y = Vec::new();

    for (i, features) in extract_features(&signal, sample_rate)
        .into_iter()
        .enumerate()
    {
        // Label each frame by the time at its center
        let center = (i * hop + frame_length / 2) as f64 / sample_rate;
        if let Some(a) = annotations
            .iter()
            .find(|a| center >= a.start && center < a.end)
        {
            x.push(features);
            y.push(a.is_speech);
        }
    }

    Ok((x, y))
}
