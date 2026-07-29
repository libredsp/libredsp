use crate::signal::Signal;
use crate::simulator::{Node, Packet};
use crate::types::GeneratorType;
use std::f64::consts::PI;

pub struct Generator {
    pub id: Option<usize>,
    pub generator_type: GeneratorType,
    buffer: Signal,       // Store the generated signal
    current_index: usize, // Track position in the buffer
}

impl Node for Generator {
    fn execute(&mut self, _: &[Packet]) -> Option<Packet> {
        // Check if we've reached the end of the buffer
        if self.current_index >= self.buffer.len() {
            // Option 1: Loop back to beginning
            self.current_index = 0;

            // Option 2: Or regenerate if needed (for white noise with different seed)
            // self.buffer = Self::generate_signal(&self.generator_type);
            // self.current_index = 0;
        }

        let value = self.buffer.data[self.current_index];
        self.current_index += 1;

        Some(Packet::new(value, self.get_id()))
    }

    fn get_display_name(&self) -> &str {
        match self.generator_type {
            GeneratorType::Sine { .. } => "Sine Generator",
            GeneratorType::PulseTrain { .. } => "Pulse Train Generator",
            GeneratorType::WhiteNoise { .. } => "White Noise Generator",
            GeneratorType::Delta { .. } => "Delta Generator",
            GeneratorType::Step { .. } => "Step Generator",
        }
    }

    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }

    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}

impl Generator {
    pub fn new(generator_type: GeneratorType) -> Self {
        let buffer = Self::generate_signal(&generator_type);

        Generator {
            id: None,
            generator_type,
            buffer,
            current_index: 0,
        }
    }

    fn generate_signal(gen_type: &GeneratorType) -> Signal {
        match gen_type {
            GeneratorType::Sine {
                n,
                amplitude,
                frequency,
                phase,
            } => {
                let mut res: Vec<f64> = Vec::with_capacity(*n);
                for i in 0..*n {
                    let t = i as f64 / *n as f64;
                    res.push(amplitude * (2.0 * PI * frequency * t + phase).sin());
                }
                Signal::new(res)
            }
            GeneratorType::PulseTrain {
                n,
                amplitude,
                frequency,
                duty_cycle,
            } => {
                let duty = duty_cycle.clamp(0.0, 1.0);
                let mut res: Vec<f64> = Vec::with_capacity(*n);

                for i in 0..*n {
                    let normalized_time = i as f64 / *n as f64;
                    let cycle_position = (frequency * normalized_time) % 1.0;

                    if cycle_position < duty {
                        res.push(*amplitude);
                    } else {
                        res.push(0.0);
                    }
                }
                Signal::new(res)
            }
            GeneratorType::WhiteNoise {
                n,
                standard_deviation,
                mean,
            } => {
                let mut res = Vec::with_capacity(*n);

                for _ in 0..*n {
                    let u1: f64 = rand::random_range(0.0..1.0);
                    let u2: f64 = rand::random_range(0.0..1.0);
                    let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                    res.push(mean + standard_deviation * z);
                }
                Signal::new(res)
            }
            GeneratorType::Delta { n, position } => {
                let mut res: Vec<f64> = Vec::with_capacity(*n);
                for i in 0..*n {
                    if i == *position {
                        res.push(1.0);
                    } else {
                        res.push(0.0);
                    }
                }
                Signal::new(res)
            }

            GeneratorType::Step {
                n,
                amplitude,
                step_index,
            } => {
                let mut res: Vec<f64> = Vec::with_capacity(*n);
                for i in 0..*n {
                    res.push(if i >= *step_index { *amplitude } else { 0.0 });
                }
                Signal::new(res)
            }
        }
    }
}
