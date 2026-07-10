use crate::simulation::{Node, Packet};
use std::collections::HashMap;

pub struct Sum {
    pub id: Option<usize>,
    signs: HashMap<usize, bool>, /* Key = source node ID, Value = true (add) / false (subtract)  */
}

impl Sum {
    pub fn new(signs: HashMap<usize, bool>) -> Self {
        Sum { id: None, signs }
    }
}

impl Node for Sum {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {
        let mut sum = 0.0;
        for packet in input {
            let sign = if *self.signs.get(&packet.src).unwrap_or(&true) {
                1.0
            } else {
                -1.0
            };

            sum += sign * packet.output;
        }
        Some(Packet::new(sum, self.get_id()))
    }
    
    fn get_display_name(&self) -> &str {
        "Sum"
    }
    
    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }
    
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}