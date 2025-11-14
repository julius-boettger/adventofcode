use std::collections::HashMap;
use petgraph::graph::{NodeIndex, DiGraph};

type WireName = &'static str;
type Signal = Option<bool>;

type NodeMap = HashMap<WireName, NodeIndex>;
type Circuit = DiGraph<Node, Signal, u32>;

struct Node {
    output_wire: WireName,
    gate_type: GateType
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.gate_type {
            GateType::Phantom(signal) =>
                signal.map_or("-", |signal|
                    if signal { "1" } else { "0" }),
            GateType::Real(gate) => match gate {
                Gate::And => "&",
                Gate::Or => "|",
                Gate::Xor => "^",
            },
        };
        write!(f, "{} => {}", value, self.output_wire)
    }
}

#[derive(PartialEq)]
enum GateType {
    Phantom(Signal),
    Real(Gate)
}

#[derive(Copy, Clone, PartialEq)]
enum Gate {
    And,
    Or,
    Xor
}

fn insert_node(circuit: &mut Circuit, node_map: &mut NodeMap, output_wire: WireName, gate_type: GateType) -> NodeIndex {
    if let Some(node_index) = node_map.get(output_wire) {
        if circuit[*node_index].gate_type == GateType::Phantom(None) && gate_type != GateType::Phantom(None) {
            circuit[*node_index].gate_type = gate_type;
        }
        *node_index
    } else {
        let node = Node { output_wire, gate_type };
        let node_index = circuit.add_node(node);
        node_map.insert(output_wire, node_index);
        node_index
    }
}

#[advent_of_code::main("24/24")]
fn main() {
    let input_parts = INPUT.split("\n\n").collect::<Vec<_>>();
    let start_value_input = input_parts[0];
    let relation_input = input_parts[1];

    let mut node_map = NodeMap::new();
    let mut circuit = Circuit::new();

    for line in start_value_input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();
        let wire_name = parts[0];
        let start_value = parts[1].parse::<u8>().unwrap() != 0;
        insert_node(&mut circuit, &mut node_map, wire_name, GateType::Phantom(Some(start_value)));
    }

    for line in relation_input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();

        let gate = match parts[1] {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!()
        };

        let input_1 = insert_node(&mut circuit, &mut node_map, parts[0], GateType::Phantom(None));
        let input_2 = insert_node(&mut circuit, &mut node_map, parts[2], GateType::Phantom(None));
        let output = insert_node(&mut circuit, &mut node_map, parts[4], GateType::Real(gate));

        circuit.add_edge(input_1, output, None);
        circuit.add_edge(input_2, output, None);
    }

    #[cfg(debug_assertions)] {
        use std::io::Write;
        let mut graph_file = std::fs::File::create("graph.dot").unwrap();
        write!(graph_file, "{:?}", petgraph::dot::Dot::new(&circuit)).unwrap();
        std::process::Command::new("dot")
            .args(["graph.dot", "-Tsvg", "-o", "graph.svg"])
            .status().unwrap();
    }

    // simulate signals starting from the starting values
    // keep track of which nodes have already written their output signal onto the wires
}
