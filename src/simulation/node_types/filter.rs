use crate::types::TransferFunction;
use crate::simulation::{Node, Packet};

/* Implementing the filtering operation:
\[
    a_0*y[n] + a_1*y[n-1] + a_2*y[n-2] + ... = b_0*x[n] + b_1*x[n-1] + b_2*x[n-2] + ...

\]
    Where $y$ is the output and $x$ is the input.
*/
pub struct Filter {
    id: Option<usize>,
    tf: TransferFunction,
    num: Vec<f64>,  // b coefficients [b0, b1, b2, ...]
    den: Vec<f64>,  // a coefficients [a0, a1, a2, ...]
    x_buffer: Vec<f64>,
    y_buffer: Vec<f64>,
    current_output: f64,
}

impl Filter {
    pub fn new(tf: TransferFunction) -> Self {
        let num = tf.num.clone();
        let den = tf.den.clone();
        let order = den.len().max(num.len());
        let x_buffer_size = order;
        let y_buffer_size = den.len();
        
        Filter {
            id: None,
            tf,
            num,
            den,
            x_buffer: vec![0.0; x_buffer_size],
            y_buffer: vec![0.0; y_buffer_size],
            current_output: 0.0,
        }
    }
    pub fn step(&mut self, input: f64) -> f64 {
        /*
            The filter equation is:
            \[
                a_0*y[n] + a_1*y[n-1] + a_2*y[n-2] + ... = b_0*x[n] + b_1*x[n-1] + b_2*x[n-2] + ...
            \]
            Where $a_0, a_1, ... a_n$ are numerator coefficients and $b_0$, $b_1$, ... $b_n$ are denomenator coefficinets.
            To compute the output $y[n]$, we should first compute the right-hand-side by multiplying the currernt input $x[m]$,
            as well as the previous x[n]'s (x[n-1], x[n-2], ...) which are stored in the x_buffer, with numerator coefficients.
            Afterwards, we compute the same thing for the left-hand-side, except the first element $a_0y[n]$, because we eventually 
            want to isolate and compute $y[n]$. So, after computing the sum for the left-hand-side, we subtract it from the sum for
            the right-hand-side, and then we simply divide by $a_0$ to get $y[n]$.
         */
        /* RHS */
        let mut x_term_sums = self.num[0] * input;  // Current input
        for j in 1..self.num.len() {
            x_term_sums += self.num[j] * self.x_buffer[j - 1];
        }
        
        /* LHS */
        let mut y_term_sums = 0.0;
        for j in 1..self.den.len() {
            y_term_sums += self.den[j] * self.y_buffer[j - 1];
        }
        
        /* Computing the output: y[n]   */
        let output = (x_term_sums - y_term_sums) / self.den[0];
        
        /* Shifting buffers */
        for j in (1..self.y_buffer.len()).rev() {
            self.y_buffer[j] = self.y_buffer[j - 1];
        }
        self.y_buffer[0] = output;
        
        for j in (1..self.x_buffer.len()).rev() {
            self.x_buffer[j] = self.x_buffer[j - 1];
        }
        self.x_buffer[0] = input;
        
        self.current_output = output;
        output
    }
}

impl Node for Filter {
    fn execute(&mut self, _: &[Packet]) -> Option<Packet> {
        None
    }
    fn get_display_name(&self) -> &str {
        "Filter"
    }
    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}