use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1
    let mut h_v = (0, 0);
    for (_, line) in reader.lines().enumerate() {
        let result = parse_h_v(&line.unwrap());
        h_v.0 += result.0;
        h_v.1 += result.1;
    }

    println!("{} * {} = {}", h_v.0, h_v.1, h_v.0 * h_v.1);

    // part 2
    let mut h_v_d = (0, 0, 0);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let result = parse_h_v_d(&line.unwrap(), h_v_d.1);

        h_v_d.0 += result.0;
        h_v_d.1 += result.1;
        if result.2 > 0 {
            h_v_d.2 += result.2;
        }
    }

    println!("{} * {} = {}", h_v_d.0, h_v_d.2, h_v_d.0 * h_v_d.2);
}

fn parse_h_v(line: &str) -> (i32, i32) {
    let direction_value: Vec<&str> = line.split(' ').collect();
    let direction = direction_value[0];
    let value = direction_value[1].parse::<i32>().unwrap();

    if direction == "forward" {
        return (value, 0);
    } else if direction == "down" {
        return (0, value);
    } else {
        return (0, value * -1);
    }
}

fn parse_h_v_d(line: &str, v: i32) -> (i32, i32, i32) {
    let direction_value: Vec<&str> = line.split(' ').collect();
    let direction = direction_value[0];
    let value = direction_value[1].parse::<i32>().unwrap();

    if direction == "forward" {
        return (value, 0, value * v);
    } else if direction == "down" {
        return (0, value, 0);
    } else {
        return (0, value * -1, 0);
    }
}
