use std::collections::HashMap;
use petgraph::{graph::{DiGraph, NodeIndex}, visit::EdgeRef};

type WireName = &'static str;
type Signal = Option<bool>;

type NodeMap = HashMap<WireName, NodeIndex>;
type Circuit = DiGraph<Node, Signal, u32>;

#[derive(Debug)]
struct Node {
    output_wire: WireName,
    output_status: WireStatus,
    gate_type: GateType,
    /// number of input wires (egdes) coming from gates (nodes)
    /// with `output_status = WireStatus::Done`
    inputs_done: u8,
}

#[derive(Debug, PartialEq)]
enum WireStatus {
    /// waiting on input signals to be determined
    Waiting,
    /// required input signals are determined, can write output signal to connected wires
    Ready,
    /// output signal has been written to connected wires
    Done
}

#[derive(Debug, PartialEq)]
enum GateType {
    Phantom(Signal),
    Logical(Gate)
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
        let output_status = match gate_type {
            GateType::Phantom(Some(_)) => WireStatus::Ready,
            _ => WireStatus::Waiting,
        };
        let node = Node { output_wire, output_status, gate_type, inputs_done: 0 };
        let node_index = circuit.add_node(node);
        node_map.insert(output_wire, node_index);
        node_index
    }
}

fn signal_char(signal: Signal) -> char {
    signal.map_or('?', |signal|
        if signal { '1' } else { '0' })
}

fn save_graph_visualization(circuit: &Circuit, name: &str) {
    #[cfg(debug_assertions)] {
        use std::io::Write;
        let dot = petgraph::dot::Dot::with_attr_getters(
            &circuit, &[],
            &|_, edge|
                format!("label=\"{}\"", signal_char(*edge.weight())),
            &|_, (_, node)| {
                let value = match node.gate_type {
                    GateType::Phantom(signal) => signal_char(signal),
                    GateType::Logical(gate) => match gate {
                        Gate::And => '&',
                        Gate::Or => '|',
                        Gate::Xor => '^',
                    },
                };
                format!("label=\"{} => {}\n{:?}\"", value, node.output_wire, node.output_status)
            },
        );
        let dot_name = format!("{name}.dot");
        let mut graph_file = std::fs::File::create(&dot_name).unwrap();
        write!(graph_file, "{dot:?}").unwrap();
        std::process::Command::new("dot")
            .args([&dot_name, "-Tsvg", "-o", &format!("{name}.svg")])
            .status().unwrap();
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
        let output = insert_node(&mut circuit, &mut node_map, parts[4], GateType::Logical(gate));

        circuit.add_edge(input_1, output, None);
        circuit.add_edge(input_2, output, None);
    }

    save_graph_visualization(&circuit, "first_graph");

    // needs to be at least as wide as the highest zXX
    // wire name number, which is 45 for my input 
    let mut result: u64 = 0;

    while circuit.node_weights().any(|n| n.output_status != WireStatus::Done) {
        let ready_nodes = circuit
            .node_indices()
            .filter(|n| circuit[*n].output_status == WireStatus::Ready)
            .collect::<Vec<_>>();
        for node in ready_nodes {
            let output_value = match circuit[node].gate_type {
                GateType::Phantom(Some(value)) => value,
                GateType::Phantom(None) => unreachable!(),
                GateType::Logical(gate) => {
                    let incoming = circuit.edges_directed(node, petgraph::Direction::Incoming).collect::<Vec<_>>();
                    debug_assert!(incoming.len() == 2);
                    let value_1 = incoming[0].weight().unwrap();
                    let value_2 = incoming[1].weight().unwrap();
                    match gate {
                        Gate::And => value_1 && value_2,
                        Gate::Or  => value_1 || value_2,
                        Gate::Xor => value_1 ^  value_2,
                    }
                },
            };

            let outgoing_edges = circuit
                .edges_directed(node, petgraph::Direction::Outgoing)
                .map(|e| e.id())
                .collect::<Vec<_>>();
            
            // the output of a "zXX" wire has been computed
            if outgoing_edges.is_empty() {
                let shift = circuit[node].output_wire.strip_prefix('z').unwrap().parse::<u8>().unwrap();
                let value = u64::from(output_value);
                result += value << shift;
            } else {
                for out_edge in outgoing_edges {
                    *circuit.edge_weight_mut(out_edge).unwrap() = Some(output_value);

                    let out_node = circuit.node_weight_mut(circuit.edge_endpoints(out_edge).unwrap().1).unwrap();
                    out_node.inputs_done += 1;
                    if out_node.inputs_done == 2 {
                        out_node.output_status = WireStatus::Ready;
                    }
                }
            }

            circuit.node_weight_mut(node).unwrap().output_status = WireStatus::Done;
        }
    }

    println!("{result}");

    save_graph_visualization(&circuit, "final_graph");
}
