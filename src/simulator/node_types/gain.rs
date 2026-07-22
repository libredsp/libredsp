use crate::simulator::{Node, Packet};

pub struct Gain {
    pub id: Option<usize>,
    gain: f64
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Gain { id: None, gain }
    }
}

impl Node for Gain {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {
        let mut res = 0.0;
        for packet in input {
            res += self.gain * packet.output;
        }

        Some(Packet::new(res, self.get_id()))
    }

    fn get_display_name(&self) -> &str {
        "Gain"
    }

    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }

    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}
