use log::{debug, info};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut digits_sum = 0;
    for line in contents.lines() {
        let patterns_value_list: Vec<&str> = line.split(" | ").collect();
        for (index, output_value) in patterns_value_list.iter().enumerate() {
            if index % 2 == 1 {
                let value_count = output_value
                    .split(" ")
                    .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                    .count();
                digits_sum += value_count;
                debug!("{:?} {:?}", value_count, digits_sum);
            }
        }
    }
    info!("part 1: {:?}", digits_sum);
}
