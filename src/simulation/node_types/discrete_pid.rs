use crate::simulation::{Node, Packet};

pub struct DiscretePID {
    id: Option<usize>,
    kp: f64,
    ki: f64,
    kd: f64,
    ts: f64,
    integral: f64,
    integral_max: f64,
    integral_min: f64,
    e_prev: f64,
}

impl DiscretePID {
    pub fn new(kp: f64, ki: f64, kd: f64, ts: f64, integral_max: f64, integral_min: f64) -> Self {
        DiscretePID {
            id: None,
            kp,
            ki,
            kd,
            ts,
            integral: 0.0,
            integral_max,
            integral_min,
            e_prev: 0.0,
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

    pub fn set_ts(&mut self, ts: f64) {
        self.ts = ts;
    }
}

impl Node for DiscretePID {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {
        let e = input.first().map(|p| p.output).unwrap_or(0.0);
        
        self.integral += e * self.ts;
        self.integral = self.clamp(self.integral);
        
        let derivative = (e - self.e_prev) / self.ts;
        
        let p_term = self.kp * e;
        let i_term = self.ki * self.integral;
        let d_term = self.kd * derivative;
        
        let output = p_term + i_term + d_term;
        
        self.e_prev = e;
        
        Some(Packet::new(output, self.get_id()))
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