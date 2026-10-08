use crate::signal::Signal;

use crate::speech::features::extract_features;
use crate::speech::frame_params;
use crate::speech::logistic_regression::LogisticRegression;

//////////////////////////////////////////////////////////////////////////////////
///////////////////////// Defining a set of data structures //////////////////////
pub struct VadConfig {
    // Specify the probability at which anything more than that we flag as 'speech'
    pub threshold: f64,
    /* In 'segment_from_probabilities' we do 'smoothing'.
     * which simply means that we look at the probabilities of the speech
     * a few frames after and before each frame and average them to compute the
     * probability that a particular frame is in fact speech or is not.
     * The following variable specifies number of frames we use for smoothing.
     */
    pub smoothing_frames: usize,
    /* The logistic regression algorithm might classify, for different frames something like:
     *
     *  [..., false, false, false, true, false, false, false, ...]
     *
     * considering this output, the 'true' is most likely a false positive, and should be discarded.
     * The follwoing variable, `min_speech`, specifies that how long should the 'true' classifications (in seconds)
     * be until they would be considered as the correct classification of speech.
     */
    pub min_speech: f64,
    /* Same story as the previous variable, except that it specifies
     * the minimum duration that a run of `false` values must have
     * before we consider it a valid non-speech region. For example consider:
     *
     *  [..., true, true, true, false, true, true, true, ...]
     */
    pub min_non_speech: f64,
}

impl Default for VadConfig {
    fn default() -> Self {
        VadConfig {
            threshold: 0.6,
            smoothing_frames: 11,
            min_speech: 0.5,
            min_non_speech: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub is_speech: bool,
}

//////////////////////////////////////////////////////////////////////////////////
///////////////////////// Defining the functions /////////////////////////////////

// The main function. Splits a recording into speech and non-speech segments.
pub fn detect_speech(
    model: &LogisticRegression,
    signal: &Signal,
    sample_rate: f64,
    config: &VadConfig,
) -> Vec<Segment> {
    let (_, hop) = frame_params(sample_rate);
    let hop_seconds = hop as f64 / sample_rate;
    let total_seconds = signal.len() as f64 / sample_rate;

    let features = extract_features(signal, sample_rate);
    let probabilities = model.predict_probabilities(&features);
    segments_from_probabilities(&probabilities, hop_seconds, total_seconds, config)
}

/// Turn per frame speech probabilities into segments covering the recording.
pub fn segments_from_probabilities(
    probabilities: &[f64],
    hop_seconds: f64,
    total_seconds: f64,
    config: &VadConfig,
) -> Vec<Segment> {
    // Number of frames
    let n = probabilities.len();

    /* We do a 'smoothing' operation in this function
     * If 'config.config_frames' is let's say, 11, then 'half' is 5 and we consider 5 frames before
     * and after each frame to compute the probability of a particular frame being speech or non speech.
     */
    let half = config.smoothing_frames / 2;

    /* Perform 'smoothing' and then apply 'thresholding' to the probabilities to decide
     * whether a frame is is speech or it's nonspeech.
     */
    let mut decisions: Vec<bool> = (0..n)
        .map(|i| {
            let lo = i.saturating_sub(half); /* A cool Rust method. It's just 'i-half' but if the result gives a negative value,
            the method returns 0 instead. */
            let hi = (i + half + 1).min(n); /* The 'min' method makes sure the largest value we get is the frame size. */
            let average = probabilities[lo..hi].iter().sum::<f64>() / (hi - lo) as f64;
            average >= config.threshold
        })
        .collect();

    /* At this point, the `decision` vector is [true, false, true, ...]
     * We need to classify a sudden false or true that occurs among consecutive
     * true or false values as false and true respectively.
     * The following code converts `config.min_speech` and `config.min_non_speech`
     * from seconds to frame numbers.
     */
    let min_non_speech = (config.min_non_speech / hop_seconds).round() as usize;
    let min_speech = (config.min_speech / hop_seconds).round() as usize;
    flip_short_identical_frames(&mut decisions, false, min_non_speech, true);
    flip_short_identical_frames(&mut decisions, true, min_speech, false);

    // Group consecutive frames with the same decision into segments
    let mut segments = Vec::new();
    let mut start = 0;
    for i in 1..=n {
        if i == n || decisions[i] != decisions[start] {
            segments.push(Segment {
                start: start as f64 * hop_seconds,
                end: i as f64 * hop_seconds,
                is_speech: decisions[start],
            });
            start = i;
        }
    }

    /* If `hop_seconds` divides `total_seconds`, there is no problem with the above code.
     * But imagine:
     *   total_seconds = 10.037
     *   hop_seconds = 0.01
     * In this case, the boundaries pushed into `segments` would be:
     *   [(0.00, 0.01), ..., (10.02, 10.03)]
     * The interval (10.03, 10.037) is not considered. The following code fixes that.
     */
    if let Some(last) = segments.last_mut() {
        last.end = total_seconds;
    }

    segments
}

/* Returns the consecutive values that have identifical values.
 * For example, the following list:
 * [false, false, true, true, true, false, false]
 * contains three 'identical segments'.
 */
fn identical_segments(decisions: &[bool]) -> Vec<(bool, usize, usize)> {
    let mut result = Vec::new();
    let mut start = 0;
    for i in 1..=decisions.len() {
        if i == decisions.len() || decisions[i] != decisions[start] {
            result.push((decisions[start], start, i));
            start = i;
        }
    }
    result
}

/* Flips (changes false to true or vice verca), the consecutive true or false values in the
 * predicted vector if their length is less than 'min_len' (in number of segments).
 */
fn flip_short_identical_frames(
    decisions: &mut [bool],
    value: bool,
    min_len: usize,
    keep_edges: bool,
) {
    for (v, start, end) in identical_segments(decisions) {
        if v != value || end - start >= min_len {
            continue;
        }
        if keep_edges && (start == 0 || end == decisions.len()) {
            continue;
        }
        for d in &mut decisions[start..end] {
            *d = !value;
        }
    }
}
