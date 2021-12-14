use log::{debug, info};
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    for line in contents.lines() {
        debug!("{:?}", line);
    }
}
