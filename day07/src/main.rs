use log::{debug, info};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut number_list: Vec<usize> = Vec::new();
    for line in contents.lines() {
        info!("{:?}", line);
        number_list = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
    }
    debug!("{:?}", number_list);

    let max: usize = *(number_list.iter().max().unwrap());
    debug!("{:?}", max);

    let mut fuel_list: Vec<i32> = vec![0; max as usize];
    for i in 0..max {
        fuel_list[i] = number_list
            .iter()
            .map(|x| (*x as i32 - i as i32).abs())
            .sum();
        debug!("{:?} {:?}", i, fuel_list[i]);
    }

    println!("part 1: {:?}", fuel_list.iter().min().unwrap());
}
