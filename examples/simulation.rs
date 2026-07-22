use libredsp::simulator::Graph;
use libredsp::simulator::node_types::{Display, Filter, Generator, Sum};
use libredsp::simulator::simulate;
use libredsp::types::{GeneratorType, TransferFunction};
use std::collections::HashMap;

/*
    Simulates a system where a discrete-time PID controls a plant with feedback.
*/
fn main() {
    let mut graph = Graph::new();

    let mut display = Display::new();
    display.set_output_file("output.csv");

    /* Clean sine */
    let sine_id = graph.add_node(Generator::new(GeneratorType::Sine {
        n: 1000,
        amplitude: 1.0,
        frequency: 5.0,
        phase: 0.0,
    }));

    /* White noise */
    let noise_id = graph.add_node(Generator::new(GeneratorType::WhiteNoise {
        n: 1000,
        mean: 0.0,
        standard_deviation: 0.1,
    }));

    /* Add sine + noise */
    let mut signs = HashMap::new();
    signs.insert(sine_id, true);
    signs.insert(noise_id, true);
    let sum_id = graph.add_node(Sum::new(signs));

    /* Low-pass FIR */
    let filter_id = graph.add_node(Filter::new(TransferFunction {
        num: vec![
            0.0550, 0.0895, 0.1208, 0.1446, 0.1575, 0.1575, 0.1446, 0.1208, 0.0895, 0.0550,
        ],
        den: vec![1.0],
    }));

    let display_id = graph.add_node(display);

    /* Connect graph */
    graph.add_edge(sine_id, sum_id).unwrap();
    graph.add_edge(noise_id, sum_id).unwrap();
    graph.add_edge(sum_id, filter_id).unwrap();
    graph.add_edge(filter_id, display_id).unwrap();

    /* Run */
    simulate(&mut graph, 1000);
}
