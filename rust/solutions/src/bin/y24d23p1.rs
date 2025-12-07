use std::collections::{HashMap, HashSet};
use petgraph::graph::{NodeIndex, UnGraph};

type Computer = &'static str;
type Network = UnGraph<Computer, (), u32>;
type ComputerMap = HashMap::<Computer, NodeIndex>;

fn insert_computer(network: &mut Network, computer_map: &mut ComputerMap, computer: Computer) -> NodeIndex {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = computer_map.get(computer) {
        *node
    } else {
        let node = network.add_node(computer);
        computer_map.insert(computer, node);
        node
    }
}

#[advent_of_code::main("24/23")]
fn main() {
    let mut network = Network::new_undirected();

    let mut computer_map = ComputerMap::new();
    for line in INPUT.lines() {
        let computer_1 = line.get(0..2).unwrap();
        let computer_2 = line.get(3..5).unwrap();
        let node_1 = insert_computer(&mut network, &mut computer_map, computer_1);
        let node_2 = insert_computer(&mut network, &mut computer_map, computer_2);
        network.add_edge(node_1, node_2, ());
    }
    drop(computer_map);

    let mut groups = HashSet::new();

    // start searching from computers that start with 't'
    for node in network.node_indices().filter(|i| network[*i].starts_with('t')) {
        let neighbors = network.neighbors(node).collect::<Vec<_>>();
        for neighbor_1 in &neighbors {
            for neighbor_2 in &neighbors {
                if neighbor_1 == neighbor_2 {
                    continue;
                }

                if network.find_edge(*neighbor_1, *neighbor_2).is_some() {
                    let mut computers = [network[node], network[*neighbor_1], network[*neighbor_2]];
                    computers.sort_unstable();
                    groups.insert(computers);
                }
            }
        }
    }

    println!("{}", groups.len());
}
