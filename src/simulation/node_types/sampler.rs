use crate::simulation::{Node, Packet};

pub struct Sampler {
    id: Option<usize>,
    held_value: f64,
}

impl Sampler {
    pub fn new() -> Self {
        Sampler {
            id: None,
            held_value: 0.0,
        }
    }
}

impl Node for Sampler {
    fn execute(&mut self, _input: &[Packet]) -> Option<Packet> {
        Some(Packet::new(self.held_value, self.get_id()))
    }

    fn update(&mut self, input: &[Packet]) {
        self.held_value = input.first().map(|p| p.output).unwrap_or(0.0);
    }

    fn output_depends_on_input(&self) -> bool {
        false
    }

    fn get_display_name(&self) -> &str {
        "Sampler"
    }
    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }

    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}
