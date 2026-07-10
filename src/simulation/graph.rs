use crate::simulation::{ Node };
use std::collections::{ HashSet, VecDeque };
pub struct Graph {
    nodes: Vec<Box<dyn Node>>,
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adj_list: Vec::new(),
        }
    }
    pub fn get_number_of_nodes(&self) -> usize {
        self.nodes.len()
    }
    pub fn get_node(&self, id: usize) -> &Box<dyn Node> {
        &self.nodes[id]
    } 
    pub fn get_adj_list(&self) -> &Vec<Vec<usize>> {
        &self.adj_list
    }

    pub fn get_node_mut(&mut self, id: usize) -> &mut Box<dyn Node> {
        &mut self.nodes[id]
    }
    pub fn add_node<T>(&mut self, mut node: T) -> usize
    where
        T: Node + 'static,
    {
        let id = self.nodes.len();
        node.set_id(id);
        self.nodes.push(Box::new(node));
        self.adj_list.push(Vec::new());
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        if from >= self.nodes.len() {
            return Err("Source node not found");
        }
        if to >= self.nodes.len() {
            return Err("Target node not found");
        }

        if !self.adj_list[from].contains(&to) {
            self.adj_list[from].push(to);
        }
        Ok(())
    }

    /*
        This function returns topological order of a graph.
        Topological order is a list containing the nodes which, for instance, if node 1 has a directed edge to node 2, node 1 appears 
        before node 2 in the order list.
        Having the topological order is a crucial step in the simulation as it find the order that each node can 'fire' without
        depending on other nodes.  
    */
    pub fn get_topological_order(&self, skip_indices: &HashSet<usize>) -> Option<Vec<usize>> {
        let n = self.nodes.len();
        let mut in_degree = vec![0; n];

        for (_from_index, neighbors) in self.adj_list.iter().enumerate() {
            for &to_idx in neighbors {
                if !skip_indices.contains(&to_idx) {
                    in_degree[to_idx] += 1;
                }
            }
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();

        let mut order = Vec::with_capacity(n);
        while let Some(idx) = queue.pop_front() {
            order.push(idx);
            for &next_idx in &self.adj_list[idx] {
                if skip_indices.contains(&next_idx) {
                    continue;
                }
                in_degree[next_idx] -= 1;
                if in_degree[next_idx] == 0 {
                    queue.push_back(next_idx);
                }
            }
        }

        if order.len() == n { Some(order) } else { None }
    }
}
