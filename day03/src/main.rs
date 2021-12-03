use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1
    let mut lines: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        lines.push(line.unwrap());
    }

    let (gamma, epsilon) = calculate_gamma_epsilon(&lines);
    println!("part 1: {} * {} = {}", gamma, epsilon, (gamma * epsilon));
}

fn calculate_gamma_epsilon(lines: &Vec<String>) -> (i32, i32) {
    // println!("{:?}", lines);

    let _row_count = lines.len();
    let _column_count = lines[0].len();

    let mut zero_one_count: Vec<(i32, i32)> = Vec::new();
    for _ in 0.._column_count {
        zero_one_count.push((0, 0));
    }

    for i in 0.._row_count {
        for (j, c) in lines[i].chars().enumerate() {
            if c == '0' {
                zero_one_count[j].0 += 1;
            } else {
                zero_one_count[j].1 += 1;
            }
        }
        // println!("{:?}", zero_one_count);
    }

    let (gamma_str, epsilon_str) = get_gamma_epsilon_string(&zero_one_count);
    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_str, 2).unwrap();
    // println!("{:?} {:?}", gamma, epsilon);

    (gamma, epsilon)
}

fn get_gamma_epsilon_string(zero_one_count: &Vec<(i32, i32)>) -> (String, String) {
    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();

    for i in 0..zero_one_count.len() {
        if zero_one_count[i].0 > zero_one_count[i].1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    (gamma.into_iter().collect(), epsilon.into_iter().collect())
}
