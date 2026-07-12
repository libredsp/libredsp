use crate::simulation::{Graph, Packet};
use std::collections::HashMap;

/* 
    The simulation function performs hybrid simulation.
    Hybrid simulation means we have interconnection of discrete-time as well continuous-time elements.
    The mechanism of the hybrid simulation algorithm is best understood through a classic example: discrete PID controlling a continuous plant.
    This setup is depicted below:

    [Gen] -> (+)[Sum] -> [D-PID] -> [Plant] -> [Display]
                 (-)                         |
                  ^--------------------------
    Here, notice that the Plant is continuous-time here and has a transfer function such as: { num: vec![2.0, 5.0], den: vec![1.0, 10.0, 2.0] }
    Now, if the deg(numerator) < deg(denominator), the plant is said to be 'strictly proper'. In general, we can convert a linear diff. eq. with
    an arbitrary transfer function to the state-space form, which gives us:
    \[ 
        x_{dot} = Ax + Bu
        y = Cx + Du
    \]
    Where, x_{dot} is a vector of unknown states and A, B, C and D are four matrices that the program has to compute. These two expressions
    tell us the output of the plant at each point in time (y = ...) and the evolution of the x_{dot} vector via \(x_{dot} = ...\).
    When we have a strictly proper system, D matrix is zero. Hence, the output of such system is determined solely via its states and without
    any effect from the input. The implementation of the plant (and all 'stateful' elements in the program) contains a method output_depends_on_input(), 
    which returns true if the output of the plant depends on the input, and 'false' if it doesn't.
    Now, let's see a step by step simulation of the D-PID that drives a Plant and has feedback.
    We assume that the plant is strictly proper here.
    Essentially, all the equations that describe our system are as follows:

    Gen:      s[n]
    Sum:      y[n] = s[n] - p[n]
    D-PID:    d[n] = PID(y[n])
    Plant:    p[n] = diffeqOutput(nTs) = Cx  
    Display:  no-output

    What you can see in these equations is that for elements whose output does depend on their input, the arrow in the graph precisely
    expresses the dependency between the equations. For example, to compute the output of the sum, y[n], we should first compute s[n] and subtract p[n]
    from it. Therefore, the algorithm basically finds the topological sort of the graph and computes the output of the nodes in that order to find
    the LHS of the nodes' equations. However, while finding the topological sort, we exclude the directed edges to the nodes whose output 
    doesn't depend on input! For this example, it is the plant. Even though, on the graph, there is an arrow from the D-PID to the Plant, the plant's
    output is determined regardless of the input it receives when we want to solve the equations above. Now, once we finish finding the LHS expressions
    in the equations, we are essentially done for one step of the simulation. For the next step of the simulation, we should find p[n+1] and d[n+1],
    which are the output of the plant and discrete-time PID in the next iteration. This is done by calling their 'update()' function that updates their
    state. One small detail to add is that while we are updating the Plant's state, we are essentially assuming that the output of the D-PID is held constant
    for the duration of Ts, which is the time interval of the plant's output sampling. In other words, p[n+1] is just diffeqOutput((n+1)Ts).
    
    Lastly, let's see what happens when the plant is not strictly proper. That means D != 0 and the plant's output depends on its input.
    The equations we had before then change to:

    Gen:      s[n]
    Sum:      y[n] = s[n] - p[n]
    D-PID:    d[n] = PID(y[n])
    Plant:    p[n] = diffeqOutput(nTs) = Cx + Dd[n] 
    Display:  no-output

    Notice that something interesting happens. To compute p[n], we need d[n]. To compute d[n] we need y[n]. And, to compute y[n] we need, again, p[n]!
    This is called an algebraic loop. To solve these values, we can't simply do the topological sort and execute nodes in that order.
    If our program sees this, it detects a cycle and aborts. One way to break the cycle is to add a simple 'delay' via the Sampler element.
*/
pub fn simulate(graph: &mut Graph, simulation_steps: usize) {
    let order = match graph.get_topological_order() {
        Some(order) => order,
        None => {
            println!("Cycle detected!");
            return;
        }
    };

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

    /* Most recent output packet per node (this step, once phase 1 below has
       run for it; previous step's value beforehand). */
    let mut outputs: HashMap<usize, Packet> = HashMap::new();
    for node_id in 0..graph.get_number_of_nodes() {
        outputs.insert(node_id, Packet::new(0.0, node_id));
    }

    /* Define zero packet to feed as input to blocks that don't have input */
    let zero_packet = Packet::new(0.0, 0);

    /* Simulation loop */
    for _ in 0..simulation_steps {
        /*
            @@@ <Step one> @@@
            For each node, in topological order, execute them and store their output in 'outputs'.
            Remember: If a node returns 'false' for 'output_depends_on_input', then it will have parent node(s)
            executing before it.
        */
                for &node_id in &order {
            let parent_ids = parents.get(&node_id).unwrap();
            let mut inputs: Vec<Packet> = parent_ids.iter()
                .filter_map(|p| outputs.get(p))
                .cloned()
                .collect();
 
            if inputs.is_empty() {
                inputs.push(zero_packet.clone());
            }
 
            if let Some(output) = graph.get_node_mut(node_id).execute(&inputs) {
                outputs.insert(node_id, output);
            }
        }

        /*
            @@@ <Step two> @@@
            Update the state of the stateful elements.            
        */
        for id in 0..graph.get_number_of_nodes() {
            let parent_ids = parents.get(&id).unwrap();
            let inputs: Vec<Packet> = parent_ids.iter()
                .filter_map(|p| outputs.get(p))
                .cloned()
                .collect();
 
            graph.get_node_mut(id).update(&inputs);
        }

    }
}