use libredsp::simulation::Graph;
    use libredsp::simulation::node_types:: {
                                        Modifier,
                                        Step,
                                        Sum,
                                        Plant,
                                        DiscretePID,
                                        Display
                                    };
use libredsp::types::TransferFunction;
use libredsp::simulation::simulate;
use std::collections::{ HashSet, HashMap };

fn main() {
    let mut graph = Graph::new();
    let mut display = Display::new();
    display.set_output_file("output.csv");

    /* Add nodes */
    let step = graph.add_node(Step::new(2.0));
    let display_id= graph.add_node(display);
    let pid = graph.add_node(DiscretePID::new(10.0, 1.0, 0.01, 0.01, 1.0, -1.0));
    let plant = graph.add_node(Plant::new(
        TransferFunction { num: vec![2.0, 5.0], den: vec![1.0, 3.0, 2.0] },
        0.01,
        0.001
    ));
    let modifier = graph.add_node(Modifier::new(0.0, 0.0));

    /* Configuring the nodes */
    let mut signs = HashMap::new();
    signs.insert(step, true);       // Step: positive (+)
    signs.insert(modifier, false);  // Feedback: negative (-)
    let sum = graph.add_node(Sum::new(signs));

    /* Add edges: Step -> Sum -> D-PID -> Plant -> Modifier -> Filter -> Sum  */
    graph.add_edge(step, sum).unwrap();
    graph.add_edge(sum, pid).unwrap();
    graph.add_edge(pid, plant).unwrap();
    graph.add_edge(plant,display_id).unwrap();
    graph.add_edge(plant,modifier).unwrap();
    graph.add_edge(modifier,sum).unwrap();

    /* Skip the continous Plant in topological sort to break the cycle */
    let mut skip_nodes = HashSet::new();
    skip_nodes.insert(plant);

    simulate(&mut graph, &skip_nodes,  100);
    
}