use std::error::Error;
use std::fs;

//////////////////////////////////////////////////////////////////////////////////
///////////////////////// Defining a set of data structures //////////////////////
#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    /* The ratio of all predictions that were correct.
    * Mathematically:
    * accuracy = (#true positive + #true negative) /
                 (#true positive + #true negative + #false positive + #false negative)
    */
    pub accuracy: f64,
    /* Of all frames that it called speech, the number of them that were actually speech.
     * Mathematically:
     * precision = (#true positive) / (#true positive + #false positive)
     */
    pub precision: f64,
    /* Of all the frames that were actually speech, the number of them that it detected.
     * Mathematically:
     * recall = (#true positive) / (#true positive + #false negative)
     *
     * Essentially, accuracy considers the ratio of non-speech and speech frames
     * and whether they were correctly or incorrectly detected, while precision
     * and recall focus only on the speech detections.
     */
    pub recall: f64,
    /* f1 combaines information from both precision and recall.
    * Mathematically:
    * f1 = 2(precision * recall) / (precision + recall)
    * The higher this value the better. If f1=1.0, it means we have the perfect precision and recall.

    */
    pub f1: f64,
}

pub struct TrainConfig {
    pub learning_rate: f64,
    /* An 'epoch' is one complete pass through the training dataset when we train the model.
     * The follwoing variable defines the number of epochs, a.k.a. passes we do through the
     * training dataset.
     */
    pub epochs: usize,
}

impl Default for TrainConfig {
    fn default() -> Self {
        TrainConfig {
            learning_rate: 0.01,
            epochs: 5000,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////
////////////////  The impl. of logistic regression starts here  //////////////////

// The 'sigmoid' function: f(x) = 1 / (1 + e^-x)
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/*  The logistic regression data structure containing the `weights`.
* When you have input data `x` as:
*     x = [x_1, x_2, x_3, ..., x_n]
* the logistic regression then computes:
*     z = w_0 + w_1x_1 + w_2x_2 + ... + w_nx_n
* and then feeds `z` to the sigmoid function to receive a value between 0 and 1.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LogisticRegression {
    pub weights: Vec<f64>,
}

impl LogisticRegression {
    /* For a given set of `weights`, it computes the probability of a given input data to occur. */
    pub fn predict_probability(&self, x: &[f64]) -> f64 {
        assert_eq!(x.len() + 1, self.weights.len(), "wrong feature count");

        let mut z = self.weights[0];
        for j in 0..x.len() {
            z += self.weights[j + 1] * x[j];
        }
        sigmoid(z)
    }

    pub fn predict_probabilities(&self, x: &[Vec<f64>]) -> Vec<f64> {
        x.iter().map(|row| self.predict_probability(row)).collect()
    }

    /* Performing the update equation on weights, `weight.epochs` times:
     * weights <- weights + alpha * \sum_{i=0}^{n-1} (y_i - h(x_i)) * x_i
     */
    pub fn train(x: &[Vec<f64>], y: &[bool], config: &TrainConfig) -> Result<Self, String> {
        if x.is_empty() || x.len() != y.len() {
            return Err("x and y must be non-empty and the same length".into());
        }
        let n = x.len();
        let dim = x[0].len();

        let mut model = LogisticRegression {
            weights: vec![0.0; dim + 1],
        };

        for _ in 0..config.epochs {
            let mut grad = vec![0.0; dim + 1];

            for i in 0..n {
                let err = if y[i] { 1.0 } else { 0.0 } - model.predict_probability(&x[i]);

                grad[0] += err; // x0 = 1
                for j in 0..dim {
                    grad[j + 1] += err * x[i][j];
                }
            }

            for j in 0..=dim {
                model.weights[j] += config.learning_rate * grad[j] / n as f64;
            }
        }

        Ok(model)
    }

    // Calculate the accuracy, precision, recall and f1
    pub fn evaluate(&self, x: &[Vec<f64>], y: &[bool], threshold: f64) -> Metrics {
        let (mut true_positive, mut false_positive, mut true_negative, mut false_negative) =
            (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);

        for (row, &label) in x.iter().zip(y) {
            match (self.predict_probability(row) >= threshold, label) {
                (true, true) => true_positive += 1.0,
                (true, false) => false_positive += 1.0,
                (false, false) => true_negative += 1.0,
                (false, true) => false_negative += 1.0,
            }
        }

        let ratio = |a: f64, b: f64| if b > 0.0 { a / b } else { 0.0 };
        let precision = ratio(true_positive, true_positive + false_positive);
        let recall = ratio(true_positive, true_positive + false_negative);

        Metrics {
            accuracy: ratio(
                true_positive + true_negative,
                true_positive + false_positive + true_negative + false_negative,
            ),
            precision,
            recall,
            f1: ratio(2.0 * precision * recall, precision + recall),
        }
    }

    // Convert weights to a one line of text
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let line: Vec<String> = self.weights.iter().map(|t| t.to_string()).collect();
        fs::write(path, line.join(" ") + "\n")?;
        Ok(())
    }
    // Convert weights from a textual format to an actual weights (with vector type)
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let weights: Vec<f64> = text
            .split_whitespace()
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;
        if weights.is_empty() {
            return Err("model file is empty".into());
        }
        Ok(LogisticRegression { weights })
    }
}
