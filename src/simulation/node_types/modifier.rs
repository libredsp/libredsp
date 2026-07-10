use crate::simulation::{Node, Packet};
use rand::random_range;

pub struct Modifier {
    id: Option<usize>,
    mean: f64,
    std_dev: f64,
}

impl Modifier {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Modifier{
            id: None,
            mean, std_dev
        }
    }
    /* 
        The Box-Muller method to generate a normally distributed random number from
        two independent uniformly distributed numbers.  */
    fn box_muller(&self) -> f64 {
        let u1: f64 = random_range(0.0..1.0);
        let u2: f64 = random_range(0.0..1.0);
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        self.mean + self.std_dev * z
    }    
}

impl Node for Modifier {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {
        let base_number: f64 = input.first().map(|p| p.output).unwrap_or(0.0);        
        Some(Packet::new(
            base_number + self.box_muller(),
            self.get_id()
        ))
    }
    fn get_display_name(&self) -> &str {
        "Modifier"
    }
    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}