use crate::simulation::{Node, Packet};

pub struct Step {
    pub id: Option<usize>,
    pub amplitude: f64,

}

impl Step {
    pub fn new(amplitude: f64) -> Self {
        Step{
            id: None,
            amplitude
        }
    }
}

impl Node for Step {
    fn execute(&mut self, _: &[Packet]) -> Option<Packet> {
        Some(Packet::new(self.amplitude, self.get_id()))
    }
    fn get_display_name(&self) -> &str {
        "Step"
    }
    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}