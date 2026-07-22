#[derive(Clone)]
pub struct Packet {
    pub output: f64,
    pub src: usize
}

impl Packet {
    pub fn new(output: f64, src: usize) -> Self {
        Packet { output, src }
    }
}