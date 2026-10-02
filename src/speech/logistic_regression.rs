#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
}

pub struct TrainConfig {
    pub learning_rate: f64,
    pub epochs: usize,
}

impl Default for TrainConfig {
    fn default() -> Self {
        TrainConfig {
            learning_rate: 0.5,
            epochs: 800,
        }
    }
}

/// Numerically stable sigmoid.
pub fn sigmoid(z: f64) -> f64 {
    if z >= 0.0 {
        1.0 / (1.0 + (-z).exp())
    } else {
        let e = z.exp();
        e / (1.0 + e)
    }
}

/// Binary logistic regression. Positive class (true) is speech.
#[derive(Debug, Clone, PartialEq)]
pub struct LogisticRegression {
    pub weights: Vec<f64>,
    pub bias: f64,
    /// Per feature mean and standard deviation from the training data.
    pub mean: Vec<f64>,
    pub std: Vec<f64>,
}

impl LogisticRegression {
    /// Probability that a frame is speech.
    pub fn predict_proba(&self, features: &[f64]) -> f64 {
        assert_eq!(features.len(), self.weights.len(), "wrong feature count");

        let mut logit = self.bias;
        for j in 0..self.weights.len() {
            let standardized = (features[j] - self.mean[j]) / self.std[j];
            logit += self.weights[j] * standardized;
        }

        sigmoid(logit)
    }

    pub fn predict_proba_all(&self, features: &[Vec<f64>]) -> Vec<f64> {
        features.iter().map(|f| self.predict_proba(f)).collect()
    }
    ///
    /// Train on feature vectors `x` with labels `y` (true = speech).
    pub fn fit(x: &[Vec<f64>], y: &[bool], config: &TrainConfig) -> Result<Self, String> {
        if x.is_empty() || x.len() != y.len() {
            return Err("x and y must be non-empty and the same length".into());
        }
        let n = x.len();
        let dim = x[0].len();

        let n_pos = y.iter().filter(|&&l| l).count();
        let n_neg = n - n_pos;
        if n_pos == 0 || n_neg == 0 {
            return Err("need both speech and non-speech frames".into());
        }

        // Mean and standard deviation of each feature
        let mut mean = vec![0.0; dim];
        for row in x {
            for j in 0..dim {
                mean[j] += row[j] / n as f64;
            }
        }
        let mut std = vec![0.0; dim];
        for row in x {
            for j in 0..dim {
                std[j] += (row[j] - mean[j]).powi(2) / n as f64;
            }
        }
        for s in &mut std {
            *s = s.sqrt();
            if *s < 1e-12 {
                *s = 1.0; // constant feature, avoid dividing by zero
            }
        }

        // Standardized copy of the data
        let z: Vec<Vec<f64>> = x
            .iter()
            .map(|row| (0..dim).map(|j| (row[j] - mean[j]) / std[j]).collect())
            .collect();

        // Weight each class so both count equally, even if one is rarer
        let w_pos = n as f64 / (2.0 * n_pos as f64);
        let w_neg = n as f64 / (2.0 * n_neg as f64);

        let mut weights = vec![0.0; dim];
        let mut bias = 0.0;

        for _ in 0..config.epochs {
            let mut grad_w = vec![0.0; dim];
            let mut grad_b = 0.0;

            for i in 0..n {
                let logit = bias + (0..dim).map(|j| weights[j] * z[i][j]).sum::<f64>();
                let p = sigmoid(logit);
                let target = if y[i] { 1.0 } else { 0.0 };
                let class_weight = if y[i] { w_pos } else { w_neg };
                let err = class_weight * (p - target);

                for j in 0..dim {
                    grad_w[j] += err * z[i][j];
                }
                grad_b += err;
            }

            // The class weights sum to n, so dividing by n gives the average
            for j in 0..dim {
                weights[j] -= config.learning_rate * grad_w[j] / n as f64;
            }
            bias -= config.learning_rate * grad_b / n as f64;
        }

        Ok(LogisticRegression {
            weights,
            bias,
            mean,
            std,
        })
    }
    ///
    /// Score the model on labeled frames. A frame counts as speech when
    /// its probability is at or above `threshold`.
    pub fn evaluate(&self, x: &[Vec<f64>], y: &[bool], threshold: f64) -> Metrics {
        let (mut tp, mut fp, mut tn, mut fn_) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);

        for (row, &label) in x.iter().zip(y) {
            let predicted = self.predict_proba(row) >= threshold;
            match (predicted, label) {
                (true, true) => tp += 1.0,
                (true, false) => fp += 1.0,
                (false, false) => tn += 1.0,
                (false, true) => fn_ += 1.0,
            }
        }

        let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
        let recall = if tp + fn_ > 0.0 { tp / (tp + fn_) } else { 0.0 };
        let f1 = if precision + recall > 0.0 {
            2.0 * precision * recall / (precision + recall)
        } else {
            0.0
        };
        Metrics {
            accuracy: (tp + tn) / (tp + fp + tn + fn_).max(1.0),
            precision,
            recall,
            f1,
        }
    }
}

use std::error::Error;
use std::fs;

impl LogisticRegression {
    /// Save as 4 lines of text: bias, weights, mean, std.
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let join = |v: &[f64]| {
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        };

        let text = format!(
            "{}\n{}\n{}\n{}\n",
            self.bias,
            join(&self.weights),
            join(&self.mean),
            join(&self.std)
        );
        fs::write(path, text)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() < 4 {
            return Err("model file is incomplete".into());
        }

        let parse = |line: &str| -> Result<Vec<f64>, std::num::ParseFloatError> {
            line.split_whitespace().map(|v| v.parse()).collect()
        };

        Ok(LogisticRegression {
            bias: lines[0].trim().parse()?,
            weights: parse(lines[1])?,
            mean: parse(lines[2])?,
            std: parse(lines[3])?,
        })
    }
}
