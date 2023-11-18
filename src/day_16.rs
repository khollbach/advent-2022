use std::collections::{HashMap, HashSet};

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::input;

#[test]
fn part_1() {
    let input = input!(16);
    let graph = parse_input(input);
    assert!(graph.is_undirected());
    // dbg!(&graph, graph.nodes.len());
}

fn parse_input(input: &str) -> Graph {
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let node = parse_line(line);
        nodes.insert(node.name.clone(), node);
    }
    Graph { nodes }
}

fn parse_line(line: &str) -> Node {
    static RE: Lazy<Regex> = Lazy::new(|| {
        let label = r"([A-Z]+)";
        let num = r"(\d+)";
        let any = r"(.*)";
        let re = format!(r"^Valve {label} has flow rate={num}; tunnels? leads? to valves? {any}$");
        Regex::new(&re).unwrap()
    });

    let caps = RE
        .captures(line)
        .with_context(|| format!("no match\n\tline: {line:?}\n\tregex: {RE:?}"))
        .unwrap();

    let name = caps[1].to_owned();
    let flow_rate = caps[2].parse().unwrap();
    let neighbors = caps[3].split(", ").map(str::to_owned).collect();

    Node {
        name,
        flow_rate,
        neighbors,
    }
}

struct Graph {
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
struct Node {
    name: String,
    flow_rate: u32,
    neighbors: HashSet<String>,
}

impl Graph {
    fn is_undirected(&self) -> bool {
        fn dfs<'a, 'b>(g: &'a Graph, curr: &'a str, seen: &'b mut HashSet<&'a str>) -> bool {
            if seen.contains(curr) {
                return true;
            }
            seen.insert(curr);

            g.nodes[curr]
                .neighbors
                .iter()
                .all(|next| g.nodes[next].neighbors.contains(curr) && dfs(g, next, seen))
        }

        let start = self.nodes.keys().next().unwrap();
        let mut seen = HashSet::new();
        dfs(self, start, &mut seen) && seen.len() == self.nodes.len()
    }

    // fn brute_force(&self) -> u32 {
    //     let mut state = State {
    //         curr_node: "AA".to_owned(),
    //         time_remaining: 30,
    //         opened: HashSet::with_capacity(self.nodes.len()),
    //     };
    //     self.recurse(&mut state)
    // }

    // fn recurse(&self, state: &mut State) -> u32 {
    //     if state.time_remaining == 0 {
    //         return 0;
    //     }

    //     let mut best = 0;

    //     if !state.opened.contains(&state.curr_node) {
    //         // First option: spend a minute opening the current valve.
    //         state.time_remaining -= 1;
    //         state.opened.insert(state.curr_node.clone());
    //         let score = state.time_remaining + self.recurse(state);
    //         state.opened.remove(&state.curr_node);
    //         state.time_remaining += 1;

    //         best = max(best, score);
    //     }

    //     // Other options: spend a minute traversing a tunnel to a nearby valve.
    //     for next in &self.nodes[&state.curr_node].neighbors {
    //         todo!()
    //     }

    //     best
    // }
}

// struct State {
//     curr_node: String,
//     time_remaining: u8,
//     opened: HashSet<String>,
// }
