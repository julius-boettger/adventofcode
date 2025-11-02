use std::collections::{HashMap, HashSet};
use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug)]
struct Network {
    graph: UnGraph<&'static str, (), u32>,
    nodes: HashMap<&'static str, NodeIndex>
}
impl Network {
    fn new() -> Self {
        Self { graph: UnGraph::new_undirected(), nodes: HashMap::new() }
    }

    fn insert(&mut self, computer: &'static str) -> NodeIndex {
        if let Some(node) = self.nodes.get(computer) {
            *node
        } else {
            let node = self.graph.add_node(computer);
            self.nodes.insert(computer, node);
            node
        }
    }

    fn insert_with_edge(&mut self, computer_1: &'static str, computer_2: &'static str) {
        let node_1 = self.insert(computer_1);
        let node_2 = self.insert(computer_2);
        self.graph.add_edge(node_1, node_2, ());
    }
}

#[advent_of_code::main("24/23")]
fn main() {
    let mut network = Network::new();
    for line in INPUT.lines() {
        let computer_1 = line.get(0..2).unwrap();
        let computer_2 = line.get(3..5).unwrap();
        network.insert_with_edge(computer_1, computer_2);
    }

    let graph = &network.graph;
    let mut groups = HashSet::new();

    // start searching from computers that start with 't'
    for node in graph.node_indices().filter(|i| graph[*i].starts_with('t')) {
        let neighbors = graph.neighbors(node).collect::<Vec<_>>();
        for neighbor_1 in &neighbors {
            for neighbor_2 in &neighbors {
                if neighbor_1 == neighbor_2 {
                    continue;
                }

                if graph.find_edge(*neighbor_1, *neighbor_2).is_some() {
                    let mut computers = [graph[node], graph[*neighbor_1], graph[*neighbor_2]];
                    computers.sort_unstable();
                    groups.insert(computers);
                }
            }
        }
    }

    println!("{}", groups.len());
}
