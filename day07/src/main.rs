use log::debug;
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut number_list: Vec<usize> = Vec::new();
    for line in contents.lines() {
        debug!("{:?}", line);
        number_list = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
    }
    debug!("{:?}", number_list);

    let max: usize = *(number_list.iter().max().unwrap());
    debug!("{:?}", max);

    let mut fuel_list: Vec<usize> = vec![0; max + 1 as usize];
    for i in 0..max + 1 {
        fuel_list[i] = number_list
            .iter()
            .map(|x| (*x as i32 - i as i32).abs() as usize)
            .sum();
        debug!("{:?} {:?}", i, fuel_list[i]);
    }
    println!("part 1: {:?}", fuel_list.iter().min().unwrap());

    for i in 0..max + 1 {
        fuel_list[i] = number_list
            .iter()
            .map(|x| get_diff((*x as i32 - i as i32).abs() as usize))
            .sum();
        debug!("{:?} {:?}", i, fuel_list[i]);
    }
    println!("part 2: {:?}", fuel_list.iter().min().unwrap());
}

fn get_diff(steps: usize) -> usize {
    ((1 + steps) * steps) / 2
}

#[test]
fn test_get_diff() {
    let expect = 0;
    let actual = get_diff(0);
    assert_eq!(expect, actual);

    let expect = 1;
    let actual = get_diff(1);
    assert_eq!(expect, actual);

    let expect = 3;
    let actual = get_diff(2);
    assert_eq!(expect, actual);

    let expect = 6;
    let actual = get_diff(3);
    assert_eq!(expect, actual);
}
