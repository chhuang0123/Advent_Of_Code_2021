use log::{debug, info};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Node<'a>(&'a str, bool);

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    for line in contents.lines() {
        let nodes: Vec<&str> = line.split('-').collect();
        // debug!("{:?}", nodes);

        let left = nodes[0];
        let right = nodes[1];

        let mut only_once_left = false;
        let mut only_once_right = false;

        if left == "start" || left == "end" || left.chars().nth(0).unwrap().is_lowercase() {
            only_once_left = true;
        }

        if right == "start" || right == "end" || right.chars().nth(0).unwrap().is_lowercase() {
            only_once_right = true;
        }

        let node_left = Node(left, only_once_left);
        let node_right = Node(right, only_once_right);

        debug!("{:?} {:?}", node_left, node_right);

        graph
            .entry(node_left)
            .or_insert(Vec::new())
            .push(node_right);
        graph
            .entry(node_right)
            .or_insert(Vec::new())
            .push(node_left);
    }
    debug!("{:?}", graph);
}
