use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn read_input() -> HashMap<i32, (HashMap<i32, HashSet<i32>>, i32, i32)> {
    let mut tests_map: HashMap<i32, (HashMap<i32, HashSet<i32>>, i32, i32)> = HashMap::new();
    let num_test_cases = read_line().trim().parse::<i32>().unwrap();
    for test_case in 0..num_test_cases {
        let mut graph_data: HashMap<i32, HashSet<i32>> = HashMap::new();
        let edges_and_nodes = read_line().trim().split(" ")
            .map(|x: &str| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let num_nodes = edges_and_nodes[0];
        let num_edges = edges_and_nodes[1];
        for _ in 0..num_edges {
            let edge = read_line().trim().split(" ")
                .map(|x: &str| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            insert_edge(&mut graph_data, edge[0], edge[1]);
            insert_edge(&mut graph_data, edge[1], edge[0]);
        }
        let start_node = read_line().trim().parse::<i32>().unwrap();
        tests_map.insert(test_case, (graph_data, start_node, num_nodes));
    }
    tests_map
}

fn insert_edge(graph_data: &mut HashMap<i32, HashSet<i32>>, src: i32, dest: i32) {
    let node_set = match graph_data.entry(src) {
        Vacant(entry) => entry.insert(HashSet::new()),
        Occupied(entry) => entry.into_mut()
    };
    node_set.insert(dest);
}

fn get_path_lengths(graph: &HashMap<i32, HashSet<i32>>,
                   start_node: &i32) -> HashMap<i32, i32> {
    let mut result_map: HashMap<i32, i32> = HashMap::new();

    let graph_data: HashMap<i32, HashSet<i32>> = graph.to_owned();
    let mut visited_nodes: HashSet<i32> = HashSet::new();
    let mut child_nodes: HashSet<i32> = HashSet::new();
    child_nodes.insert(*start_node);
    let mut remaining_nodes: HashSet<i32> = child_nodes.to_owned();
    let mut distance: i32 = 0;
    while !remaining_nodes.is_empty() {
        let mut expand_paths: HashSet<i32> = HashSet::new();
        for node in child_nodes.iter() {
            result_map.entry(*node).or_insert_with(|| { distance });
            expand_paths = match graph_data.get(node) {
                Some(node_set) => node_set.union(&expand_paths).cloned().collect(),
                None => HashSet::new()
            }
        }
        visited_nodes = visited_nodes.union(&child_nodes).cloned().collect();
        child_nodes = expand_paths;
        remaining_nodes = child_nodes.difference(&visited_nodes).cloned().collect();
        distance += 1;
    }
    result_map
}

fn main () {
    let tests = read_input();
    for i in 0..tests.len() {
        let test = tests.get(&(i as i32)).unwrap().to_owned();
        let (graph, start_node, num_nodes) = test;
        let path_lengths = get_path_lengths(&graph, &start_node);
        for dest_node in 1..num_nodes + 1 {
            if start_node != dest_node {
                match path_lengths.get(&dest_node) {
                    Some(v) => print!("{} ", 6 * v),
                    None => print!("{} ", -1)
                }
            }
        }
        println!("");
    }
}
