extern crate permutohedron;
extern crate petgraph;
extern crate regex;

use permutohedron::Heap;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Graph, Undirected};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let re = Regex::new(r"^(?P<a>[A-Za-z]+) to (?P<b>[A-Za-z]+) = (?P<dist>\d+)$").unwrap();
    let input = fs::read_to_string("inputs/day9.txt").unwrap();

    let line_count = input.lines().count();

    let mut all_nodes: HashMap<&str, NodeIndex<DefaultIx>> = HashMap::new();
    let mut graph: Graph<&str, u32, Undirected, DefaultIx> =
        Graph::with_capacity(line_count, line_count);

    for l in input.lines() {
        let caps = re.captures(l.trim()).unwrap();
        let a = caps.name("a").unwrap().as_str();
        let b = caps.name("b").unwrap().as_str();
        let dist: u32 = caps.name("dist").unwrap().as_str().parse().unwrap();

        {
            all_nodes.entry(a).or_insert_with(|| graph.add_node(a));
            all_nodes.entry(b).or_insert_with(|| graph.add_node(b));
        }

        let node_a = all_nodes.get(&a).unwrap();
        let node_b = all_nodes.get(&b).unwrap();

        graph.add_edge(*node_a, *node_b, dist);
    }

    let mut keys: Vec<&str> = Vec::new();
    for key in all_nodes.keys() {
        keys.push(*key);
    }

    let heap = Heap::new(&mut keys);
    let mut permutations: Vec<Vec<&str>> = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut distances: Vec<u32> = Vec::with_capacity(permutations.len());
    for path in permutations {
        let mut i = 0;
        let mut dist: u32 = 0;
        let mut full_path = true;

        while i < path.len() - 1 {
            let node_a = *all_nodes.get(path[i]).unwrap();
            let node_b = *all_nodes.get(path[i + 1]).unwrap();
            if let Some(edge) = graph.find_edge(node_a, node_b) {
                dist += graph.edge_weight(edge).unwrap();
                i += 1;
            } else {
                full_path = false;
                break;
            }
        }

        if full_path {
            distances.push(dist);
        }
    }

    distances.sort();

    println!("minimum path: {}", distances.first().unwrap());
    println!("maximum path: {}", distances.last().unwrap());
}
