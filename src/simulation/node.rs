use crate::simulation::Packet;

pub trait Node {
    /* 
        Execute the node to produce this step's output. If
       has_direct_feedthrough() is false, this MUST NOT depend on `input`
       (it may be stale or not-yet-produced this step) -- only on internal
       state. If true, `input` is guaranteed to hold this step's freshly
       computed upstream values.
    */
    fn execute(&mut self, input: &[Packet]) -> Option<Packet>;
    /* 
        Returns 'true' if output of the block depends on input. Ex: Summer block.
        Otherwise, it returns 'false'. An example of a block that its output doesn't
        depend on its input is a plant that is "strinctly proper", i.e, deg(Numeraetor) < deg(Denomenator).
        The state-space equations for a linear diff. eq. system in general:
        \[
            x_{dot} = Ax + Bu
            y = Cx + Du
        \]
        If the system is strictly proper, the matrix D is zero.
        Hence, the plant's output at a given time only depends on its current state.
     */
    fn output_depends_on_input(&self) -> bool {
        true
    }
    /*
        Nodes that are stateful has to have an 'update' function that update the node's state.
        Example of such systems are PID, Plant and Filter.
    */
    fn update(&mut self, _input: &[Packet]) {}

    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn get_display_name(&self) -> &str;
}

