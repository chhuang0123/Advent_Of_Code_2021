use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1 + 2
    let mut h_v_d = (0, 0, 0);
    for (_, line) in reader.lines().enumerate() {
        let result = parse_h_v_d(&line.unwrap(), h_v_d.1);

        h_v_d.0 += result.0;
        h_v_d.1 += result.1;
        if result.2 > 0 {
            h_v_d.2 += result.2;
        }
    }

    println!("part 1: {} * {} = {}", h_v_d.0, h_v_d.1, h_v_d.0 * h_v_d.1);
    println!("part 2: {} * {} = {}", h_v_d.0, h_v_d.2, h_v_d.0 * h_v_d.2);
}

fn parse_h_v_d(line: &str, v: i32) -> (i32, i32, i32) {
    let direction_value: Vec<&str> = line.split(' ').collect();
    let direction = direction_value[0];
    let value = direction_value[1].parse::<i32>().unwrap();

    if direction == "forward" {
        (value, 0, value * v)
    } else if direction == "down" {
        (0, value, 0)
    } else {
        (0, value * -1, 0)
    }
}
