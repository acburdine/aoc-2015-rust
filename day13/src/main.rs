extern crate permutohedron;
extern crate petgraph;
extern crate regex;

use permutohedron::Heap;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Directed, Graph};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn max_happiness(
    nodes: &HashMap<&str, NodeIndex<DefaultIx>>,
    graph: &Graph<&str, i32, Directed, DefaultIx>,
) -> i32 {
    // note: this is incredibly sub-optimal since the seating chart graph is cyclic
    // rather than acyclic, meaning that we'll compute the same seating chart multiple times.
    // Shouldn't affect the end result, so I'm lazy for the moment and just doing permutations
    let mut keys: Vec<&str> = nodes.keys().map(|v| *v).collect();
    let heap = Heap::new(&mut keys);
    let mut permutations: Vec<Vec<&str>> = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut happiness: Vec<i32> = Vec::with_capacity(permutations.len());
    for path in permutations {
        let mut i = 0;
        let len = path.len();

        let mut score: i32 = 0;

        while i < len {
            let node_a = *nodes.get(path[i]).unwrap();
            let node_b = *nodes.get(path[(i + 1) % len]).unwrap(); // handle wraparound since the path is cyclic

            score += graph
                .edges_connecting(node_a, node_b)
                .map(|e| e.weight())
                .sum::<i32>();
            score += graph
                .edges_connecting(node_b, node_a)
                .map(|e| e.weight())
                .sum::<i32>();
            i += 1;
        }

        happiness.push(score);
    }

    happiness.sort();
    *happiness.last().unwrap()
}

fn main() {
    let re = Regex::new(r"^(?P<name>[A-Za-z]+) would (?P<mod>gain|lose) (?P<hap>\d+) happiness units by sitting next to (?P<next>[A-Za-z]+)\.$").unwrap();
    let input = fs::read_to_string("inputs/day13.txt").unwrap();

    let line_count = input.lines().count();

    let mut all_nodes: HashMap<&str, NodeIndex<DefaultIx>> = HashMap::new();
    let mut graph: Graph<&str, i32, Directed, DefaultIx> =
        Graph::with_capacity(line_count, line_count);

    for l in input.lines() {
        let caps = re.captures(l.trim()).unwrap();
        let name = caps.name("name").unwrap().as_str();
        let gain_lose = caps.name("mod").unwrap().as_str();
        let mut hap: i32 = caps.name("hap").unwrap().as_str().parse().unwrap();
        let next = caps.name("next").unwrap().as_str();

        if gain_lose.eq_ignore_ascii_case("lose") {
            hap *= -1;
        }

        {
            all_nodes
                .entry(name)
                .or_insert_with(|| graph.add_node(name));
            all_nodes
                .entry(next)
                .or_insert_with(|| graph.add_node(next));
        }

        let node_name = all_nodes.get(&name).unwrap();
        let node_next = all_nodes.get(&next).unwrap();

        graph.add_edge(*node_name, *node_next, hap);
    }

    println!("maximum happiness: {}", max_happiness(&all_nodes, &graph));

    let node_me = graph.add_node("me");
    for (_, node) in all_nodes.iter() {
        graph.add_edge(node_me, *node, 0);
    }

    all_nodes.insert("me", node_me);
    println!(
        "maximum happiness with me: {}",
        max_happiness(&all_nodes, &graph)
    )
}
