use log::{debug, info};
use std::collections::{BTreeMap};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in contents.lines() {
        let nodes: Vec<&str> = line.split('-').collect();
        // debug!("{:?}", nodes);

        let left = nodes[0];
        let right = nodes[1];

        graph
            .entry(left.to_string())
            .or_insert(Vec::new())
            .push(right.to_string());
        graph
            .entry(right.to_string())
            .or_insert(Vec::new())
            .push(left.to_string());
    }
    debug!("{:?}", graph);

    let mut visited: Vec<String> = Vec::new();
    let mut all_path: Vec<Vec<String>> = Vec::new();
    find_all_path(&graph, "start".to_string(), &mut visited, &mut all_path);

    debug!("all_path {:?}", all_path);
    info!("part 1: {}", all_path.len());
}

fn find_all_path(
    graph: &BTreeMap<String, Vec<String>>,
    from: String,
    visited: &mut Vec<String>,
    all_path: &mut Vec<Vec<String>>,
) {
    visited.push(from.clone());
    debug!("visited {:?}", visited);
    if from == "end" {
        all_path.push(visited.clone());
    }

    for nodes in graph.get(&from) {
        for node in nodes {
            let only_once = node.chars().nth(0).unwrap().is_lowercase();
            if only_once && visited.contains(node) {
                continue;
            }

            find_all_path(
                &graph,
                node.clone().to_string(),
                &mut visited.clone(),
                all_path,
            );
        }
    }
}
