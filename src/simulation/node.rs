use crate::simulation::Packet;

pub trait Node {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet>;
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn get_display_name(&self) -> &str;
}
