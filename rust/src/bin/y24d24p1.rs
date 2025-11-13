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

enum GateType {
    Phantom(Signal),
    Real(Gate)
}

#[derive(Debug)]
enum Gate {
    And,
    Or,
    Xor
}
use Gate::{And, Or, Xor};

struct Connection {
    input_1: WireName,
    input_2: WireName,
    gate: Gate,
    output: WireName,
}

fn insert_start_value(circuit: &mut Circuit, node_map: &mut NodeMap, wire_name: WireName, start_value: bool) {
    if node_map.get(wire_name).is_none() {
        let node = Node {
            output_wire: wire_name,
            gate_type: GateType::Phantom(Some(start_value))
        };
        let node_index = circuit.add_node(node);
        node_map.insert(wire_name, node_index);
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
        insert_start_value(&mut circuit, &mut node_map, wire_name, start_value);
    }

    let connections = relation_input.lines().map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        Connection {
            input_1: parts[0],
            input_2: parts[2],
            output: parts[4],
            gate: match parts[1] {
                "AND" => And,
                "OR" => Or,
                "XOR" => Xor,
                _ => unreachable!()
            }
        }
    }).collect::<Vec<_>>();

    // - register all nodes mentioned in connections with GateType::Phantom(None)
    // - register actual connections as edges in graph, possibly update node gate types 
    // - simulate signals starting from the starting values
}
