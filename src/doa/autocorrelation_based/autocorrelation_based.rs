use crate::signal::Signal;

#[derive(Debug, Clone, Copy)]
pub struct DoaConfig {
    pub sample_rate: f64,
    /* Distance between the two microphones, in meters. */
    pub mic_distance: f64,
    pub speed_of_sound: f64,
    pub frame_size: usize,
    pub hop_size: usize,
    /* Frames quieter than this are marked as inactive. */
    pub activation_threshold_dbfs: f64,
    /* Delay (in samples) measured with the source directly in front of the
     * microphones. It is subtracted from every estimated delay. */
    pub calibration_delay_samples: f64,
}

impl Default for DoaConfig {
    fn default() -> Self {
        DoaConfig {
            sample_rate: 16000.0,
            mic_distance: 0.08,
            speed_of_sound: 343.0,
            frame_size: 800, // 800 / f_s = 50 ms
            hop_size: 400,   // 400 / f_s = 25 ms
            activation_threshold_dbfs: -45.0,
            calibration_delay_samples: -1.02,
        }
    }
}

impl DoaConfig {
    /* The largest delay that is physically can happen.
     * This happens when the sound is generated parallel to the mics.
     * Hence, the difference between the distance, d, is exactly the
     * physical distance between the two mics. We use this value as
     * a small optimization to prevent computing the commplete cross-
     * correlation between the two channels. */
    pub fn max_lag(&self) -> usize {
        (self.sample_rate * self.mic_distance / self.speed_of_sound).ceil() as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DoaEstimate {
    /* Degrees from the microphone axis. 90 degress means source recieving perpendicular on the mics. */
    pub angle_degrees: Option<f64>,
    pub dbfs: f64,
}

fn remove_mean(x: &[f64]) -> Vec<f64> {
    let mean = x.iter().sum::<f64>() / x.len() as f64;
    x.iter().map(|v| v - mean).collect()
}

/* Estimates the delay, in fractional samples, of the right channel relative to the left channel.
 * A positive value means the right channel is a delayed copy of the left channel.
 */
pub fn estimate_delay(left: &Signal, right: &Signal, config: &DoaConfig) -> f64 {
    assert_eq!(left.len(), right.len());
    let n = left.len();
    let max_lag = config.max_lag() as isize;
    let offset = config.calibration_delay_samples.round() as isize;

    /* Compute enough lags to cover that range, plus one extra on each side so the
     * parabola always has both neighbors. */
    let ext = (max_lag + offset.abs() + 1) as usize;
    assert!(n > ext, "frame is too short");

    /* Removing the mean greatly improved the estimate */
    let left = Signal::new(remove_mean(left.as_slice()));
    let right = Signal::new(remove_mean(right.as_slice()));

    let lags: Vec<isize> = (-(ext as isize)..=ext as isize).collect();
    let corr = left.crosscorrelation_two_sided(&right, ext);

    /* Only accept peaks that a real source could produce after calibration. */
    let mut peak = 0;
    let mut best = f64::NEG_INFINITY;
    for i in 0..corr.len() {
        if (lags[i] - offset).abs() <= max_lag && corr[i] > best {
            best = corr[i];
            peak = i;
        }
    }
    let mut delay = lags[peak] as f64;

    /*
    We now need to take the peak value and one value before and after that,
    and find the peak value of the 'parabola' that passes through the three points.
    Why is this needed? Imagine the tree points are [8, 8.6, 8.5]. Since, the
    points 2 and 3 are closer to each other than 1 and 2, we can conclude the peak
    was between 2 and 3. That's why we take the peak of the paraboalo that passes through
    these three points.
    We would only take the middle value if the distance between that and its neighboring
    points are exactly the same, like [8, 8.5, 8].
    */
    let (y1, y2, y3) = (corr[peak - 1], corr[peak], corr[peak + 1]);
    let denominator = y1 - 2.0 * y2 + y3;
    if denominator.abs() > 1e-12 {
        delay += 0.5 * (y1 - y3) / denominator;
    }

    delay
}

/* Estimates the direction of arrival, in degrees from the microphone axis (0 to 180,
 * 90 being straight ahead), from the two channels of one frame. */
pub fn estimate_doa(left: &Signal, right: &Signal, config: &DoaConfig) -> f64 {
    let delay = estimate_delay(left, right, config) - config.calibration_delay_samples;

    let x = -config.speed_of_sound * delay / (config.sample_rate * config.mic_distance);
    x.clamp(-1.0, 1.0).acos().to_degrees()
}

/* Level of the frame in dB relative to full scale of a signed 16 bit sample. */
pub fn calculate_dbfs(left: &Signal, right: &Signal) -> f64 {
    assert_eq!(left.len(), right.len());
    let left = remove_mean(left.as_slice());
    let right = remove_mean(right.as_slice());

    let mean_square = left
        .iter()
        .zip(&right)
        .map(|(l, r)| (l * l + r * r) / 2.0)
        .sum::<f64>()
        / left.len() as f64;

    20.0 * (mean_square.sqrt() / 32768.0).max(1e-10).log10()
}

/*
 * DOA Estimator that performs DOA estimation on a stream of data.
 * The straem of data refers to the reading from two different mics.
 * Here, the samples of the two different mics are stored in the 'left'
 * and' right vectors. This name was chosen because the data of two mics
 * are typically recieved form an com. method like i2s where the left
 * channel is dedicated for one mic and the right channel is dedicated
 * for the other one.
 */
pub struct DoaEstimator {
    config: DoaConfig,
    left: Signal,
    right: Signal,
}

impl DoaEstimator {
    pub fn new(config: DoaConfig) -> Self {
        assert!(config.hop_size > 0 && config.hop_size <= config.frame_size);
        DoaEstimator {
            config,
            left: Signal::new(Vec::with_capacity(config.frame_size)),
            right: Signal::new(Vec::with_capacity(config.frame_size)),
        }
    }

    pub fn push(&mut self, left: &[f64], right: &[f64]) -> Vec<DoaEstimate> {
        assert_eq!(left.len(), right.len());
        let mut out = Vec::new();
        for (&l, &r) in left.iter().zip(right) {
            self.push_sample(l, r, &mut out);
        }
        out
    }

    fn push_sample(&mut self, l: f64, r: f64, out: &mut Vec<DoaEstimate>) {
        self.left.push(l);
        self.right.push(r);

        if self.left.len() == self.config.frame_size {
            out.push(Self::estimate_frame(&self.config, &self.left, &self.right));

            /* Slide the window.
             * Drop the oldest hop_size samples.
             */
            self.left.drop_front(self.config.hop_size);
            self.right.drop_front(self.config.hop_size);
        }
    }

    pub fn push_interleaved_i16(&mut self, samples: &[i16]) -> Vec<DoaEstimate> {
        assert!(
            samples.len() % 2 == 0,
            "stereo data must have an even length"
        );
        let mut out = Vec::new();
        for pair in samples.chunks_exact(2) {
            self.push_sample(pair[0] as f64, pair[1] as f64, &mut out);
        }
        out
    }

    pub fn config(&self) -> &DoaConfig {
        &self.config
    }

    /* Performs DOA for a frame */
    fn estimate_frame(config: &DoaConfig, left: &Signal, right: &Signal) -> DoaEstimate {
        let dbfs = calculate_dbfs(left, right);
        let angle_degrees = if dbfs < config.activation_threshold_dbfs {
            None
        } else {
            Some(estimate_doa(left, right, config))
        };
        DoaEstimate {
            angle_degrees,
            dbfs,
        }
    }
}
