use crate::math::tf_to_ss::tf_to_ss;
use crate::simulation::{Node, Packet};
use crate::types::TransferFunction;
use nalgebra::{DMatrix, DVector};

pub struct Plant {
    id: Option<usize>,
    tf: TransferFunction,
    dt: f64,
    steps_per_sample: usize,

    /* State-space matrices */
    a: Option<DMatrix<f64>>,
    b: Option<DMatrix<f64>>,
    c: Option<DMatrix<f64>>,
    d: Option<f64>,

    /* State vector (containing different state variables), and other parameters */
    x: DVector<f64>,
    t: f64,
    outputs: Vec<f64>,
    times: Vec<f64>,
    sampled_output: Vec<f64>,
    sampled_times: Vec<f64>,
    states: Vec<DVector<f64>>,
    sub_step_count: usize,
}

impl Plant {
    pub fn new(tf: TransferFunction, sampling_period: f64, dt: f64) -> Self {
        let steps_per_sample = (sampling_period / dt).round() as usize;
        /*
            State dimension, i.e. the number of variables in vector x,
            is equal to the power of the largest s-term in the denominator of the transfer function.
            Why? Because if we have, for example, the following linear constant coef. diff. eq:
            \[
                a_0y'' + a_1y' + a_2y = b_0u
            \]
            This is a second-order ODE (highest derivative is 2). To solve it, we need 2 initial
            conditions: y(0) and y'(0). These become the 2 state variables:

            \[
            x_1 = y
            x_2 = y'
            \]

            The state dimension is 2, which is order of the ODE, and that is the highest power in denominator.
        */
        let n = tf.den.len() - 1;

        let mut plant = Plant {
            id: None,
            tf,
            dt,
            steps_per_sample,
            a: None,
            b: None,
            c: None,
            d: None,
            x: DVector::zeros(n),
            t: 0.0,
            outputs: vec![0.0],
            times: vec![0.0],
            sampled_output: Vec::new(),
            sampled_times: Vec::new(),
            states: Vec::new(),
            sub_step_count: 0,
        };

        plant.init();
        plant
    }

    fn init(&mut self) {
        /* Convert transfer function to state-space */
        let (a, b, c, d) = tf_to_ss(&self.tf.num, &self.tf.den);
        let n = a.nrows();
        self.a = Some(a);
        self.b = Some(b);
        self.c = Some(c);
        self.d = Some(d);
        self.x = DVector::zeros(n);
        self.states = vec![self.x.clone()];
        self.t = 0.0;
        self.outputs = vec![0.0];
        self.times = vec![0.0];
        self.sampled_output = Vec::new();
        self.sampled_times = Vec::new();
        self.sub_step_count = 0;
    }

    pub fn set_ts(&mut self, ts: f64) {
        self.steps_per_sample = (ts / self.dt).round() as usize;
    }

    pub fn set_tf(&mut self, tf: TransferFunction) {
        self.tf = tf;
        self.init();
    }
}

impl Node for Plant {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {
        let input_val = input.first().map(|p| p.output).unwrap_or(0.0);

        let c = self.c.as_ref().unwrap();
        let d = self.d.unwrap_or(0.0);

        // Output equation: y = Cx + Du
        let y_state = c * &self.x;
        let y = y_state[0] + d * input_val;

        self.outputs.push(y);
        self.times.push(self.t);

        Some(Packet::new(y, self.get_id()))
    }

    fn update(&mut self, input: &[Packet]) {
        let last_input = input.first().map(|p| p.output).unwrap_or(0.0);

        let a = self.a.as_ref().unwrap();
        let b = self.b.as_ref().unwrap();

        for _ in 0..self.steps_per_sample {
            // Euler integration
            let x_dot = a * &self.x + b * DVector::from_element(1, last_input);

            self.x += self.dt * x_dot;

            self.states.push(self.x.clone());

            self.t += self.dt;
            self.sub_step_count += 1;
        }

        let c = self.c.as_ref().unwrap();
        let d = self.d.unwrap_or(0.0);

        let y_state = c * &self.x;
        let y = y_state[0] + d * last_input;

        self.sampled_output.push(y);
        self.sampled_times.push(self.t);
    }
     
    /*
        If the D vector is zero, then the output is determined by the state alone,
        i.e., y(t) = Cx(t)
    */
    fn output_depends_on_input(&self) -> bool {

        self.d.map_or(false, |d| d != 0.0)
    }

    fn get_display_name(&self) -> &str {
        "Plant"
    }

    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }

    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}
