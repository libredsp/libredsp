use libredsp::simulation::Graph;
use libredsp::simulation::node_types::{
    Modifier,
    Step,
    Sum,
    Plant,
    DiscretePID,
    Display,
};
use libredsp::types::TransferFunction;
use libredsp::simulation::simulate;
use std::collections::HashMap;

/*
    Simulates a system where a discrete-time PID controls a plant with feedback.
*/
fn main() {
    let mut graph = Graph::new();
    let mut display = Display::new();
    display.set_output_file("output.csv");

    /* Add nodes */
    let step_id = graph.add_node(Step::new(2.0));
    let display_id = graph.add_node(display);
    let pid_id = graph.add_node(DiscretePID::new(1.0, 10.0, 0.01, 0.01, 1.0, -1.0));
    let plant_id = graph.add_node(Plant::new(
        TransferFunction { num: vec![2.0, 5.0], den: vec![1.0, 3.0, 2.0] },
        0.01,
        0.001
    ));
    
    let modifier_id = graph.add_node(Modifier::new(0.0, 0.2));

    /* Configuring the nodes */
    let mut signs = HashMap::new();
    signs.insert(step_id, true);       // Step: positive (+)
    signs.insert(modifier_id, false);  // Feedback: negative (-)
    let sum_id = graph.add_node(Sum::new(signs));

    
    graph.add_edge(step_id, sum_id).unwrap();
    graph.add_edge(sum_id, pid_id).unwrap();
    graph.add_edge(pid_id, plant_id).unwrap();
    graph.add_edge(plant_id, display_id).unwrap();
    graph.add_edge(plant_id, modifier_id).unwrap();
    graph.add_edge(modifier_id, sum_id).unwrap();

    simulate(&mut graph, 1000);
}
