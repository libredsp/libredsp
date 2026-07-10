use std::collections::{HashMap, HashSet};
use crate::simulation::{ Graph, Packet };

pub fn simulate(
    graph: &mut Graph,
    skip_nodes: &HashSet<usize>,
    simulation_steps: usize) 
{
    
    let order = match graph.get_topological_order(&skip_nodes) {
        Some(order) => order,
        None => {
            println!("Cycle detected!");
            return;
        }
    };
    
    /* Print the order in topo: */
    // for i in 0..order.len() {
    //     println!("{}", graph.get_node_mut(order[i]).get_display_name());
    // }

    /* Create a map that maps each node (via its ID) to its parent(s) */
    let mut parents: HashMap<usize, Vec<usize>> = HashMap::new();
    for node_id in 0..graph.get_number_of_nodes() {
        parents.insert(node_id, Vec::new());
    }
    for (from, neighbors) in graph.get_adj_list().iter().enumerate() {
        for &to in neighbors {
            parents.get_mut(&to).unwrap().push(from);
        }
    }

    /* Store outputs */
    let mut outputs: HashMap<usize, Vec<Packet>> = HashMap::new();
    for node_id in 0..graph.get_number_of_nodes() {
        outputs.insert(node_id, vec![Packet::new(0.0, node_id)]);
    }

    /* Define zero packet to feed as input to blocks that don't have input */
    let zero_packet = Packet::new(0.0, 0);

    /* Simulation loop */
    for _ in 0..simulation_steps {
        for &node_id in &order {
            /* Gather inputs from parents. */
            let parent_ids = parents.get(&node_id).unwrap();
            let mut inputs: Vec<&Packet> = Vec::new();
            
            for &parent_id in parent_ids {
                if let Some(parent_outputs) = outputs.get(&parent_id) {
                    if let Some(last_output) = parent_outputs.last() {
                        inputs.push(last_output);
                    }
                }
            }

            /* If there is no inputs, use the zero packet. */ 
            if inputs.is_empty() {
                inputs.push(&zero_packet);
            }

            /* Execute node  */
            let cloned_input: Vec<Packet> = inputs.iter().map(|p| (*p).clone()).collect();
            if let Some(output) = graph.get_node_mut(node_id).execute(&cloned_input) {
                outputs.get_mut(&node_id).unwrap().push(output);
            }
        }
    }

}
