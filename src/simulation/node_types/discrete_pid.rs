use crate::simulation::{Node, Packet};

pub struct DiscretePID {
    id: Option<usize>,
    kp: f64,
    ki: f64,
    kd: f64,
    t_s: f64,

    // Controller states
    integral: f64,
    e_prev: f64,

    integral_max: f64,
    integral_min: f64,
}

impl DiscretePID {
    pub fn new(kp: f64, ki: f64, kd: f64, t_s: f64, integral_max: f64, integral_min: f64) -> Self {
        Self {
            id: None,
            kp,
            ki,
            kd,
            t_s,
            integral: 0.0,
            e_prev: 0.0,
            integral_max,
            integral_min,
        }
    }

    fn clamp(&self, value: f64) -> f64 {
        if value > self.integral_max {
            self.integral_max
        } else if value < self.integral_min {
            self.integral_min
        } else {
            value
        }
    }

    pub fn init(&mut self) {
        self.integral = 0.0;
        self.e_prev = 0.0;
    }

    pub fn set_t_s(&mut self, t_s: f64) {
        self.t_s = t_s;
    }
}

impl Node for DiscretePID {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {

        let e = input.first().map(|p| p.output).unwrap_or(0.0);
        let integral = self.clamp(self.integral + e * self.t_s);
        let derivative = (e - self.e_prev) / self.t_s;

        let output = self.kp * e + self.ki * integral + self.kd * derivative;

        Some(Packet::new(output, self.get_id()))
    }
 
    fn update(&mut self, input: &[Packet]) {
        let e = input.first()
            .map(|p| p.output)
            .unwrap_or(0.0);
 
        self.integral = self.clamp(self.integral + e * self.t_s);
        self.e_prev = e;
    }
    fn get_display_name(&self) -> &str {
        "D-PID"
    }

    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }

    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}