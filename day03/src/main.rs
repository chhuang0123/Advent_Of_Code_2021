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

    let zero_one_count: Vec<(i32, i32)> = get_zero_one_count(&lines);
    // println!("{:?}", zero_one_count);

    let (gamma, epsilon) = calculate_gamma_epsilon(&zero_one_count);
    println!("part 1: {} * {} = {}", gamma, epsilon, (gamma * epsilon));

    // part 2
    let oxygen = calculate_oxygen_co2(&lines);
}

fn get_zero_one_count(lines: &Vec<String>) -> Vec<(i32, i32)> {
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

    zero_one_count
}

fn calculate_gamma_epsilon(zero_one_count: &Vec<(i32, i32)>) -> (i32, i32) {
    let (gamma_str, epsilon_str) = get_gamma_epsilon_string(zero_one_count);
    // println!("{:?} {:?}", gamma_str, epsilon_str);

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

fn calculate_oxygen_co2(lines: &Vec<String>) {
    let oxygen_str = get_oxygen_string(&lines);
    let oxygen = i32::from_str_radix(&oxygen_str, 2).unwrap();

    let co2_str = get_co2_string(&lines);
    let co2 = i32::from_str_radix(&co2_str, 2).unwrap();

    println!("part 2: {} * {} = {}", oxygen, co2, oxygen * co2);
}

fn get_oxygen_string(lines: &Vec<String>) -> String {
    let mut zero_one_count = get_zero_one_count(&lines);
    let mut result: Vec<String> = Vec::new();
    for i in 0..zero_one_count.len() {
        // println!(
        //     "i: {}, 0: {}, 1: {} ",
        //     i, zero_one_count[i].0, zero_one_count[i].1
        // );

        let target_char: char;
        if zero_one_count[i].1 >= zero_one_count[i].0 {
            target_char = '1';
        } else {
            target_char = '0';
        }

        if result.len() == 0 {
            result = get_match_string(&lines, i, target_char);
        } else {
            result = get_match_string(&result, i, target_char);
        }
        zero_one_count = get_zero_one_count(&result);

        if result.len() == 1 {
            break;
        }
    }

    result[0].clone()
}

fn get_match_string(lines: &Vec<String>, index: usize, target_char: char) -> Vec<String> {
    // println!("index {:?}", index);
    // println!("target_char {:?}", target_char);

    let mut result: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        if target_char == (&*lines[i]).chars().nth(index).unwrap() {
            result.push(lines[i].clone());
        }
    }
    // println!("match result: {:?}", result);

    result
}

fn get_co2_string(lines: &Vec<String>) -> String {
    let mut zero_one_count = get_zero_one_count(&lines);
    let mut result: Vec<String> = Vec::new();
    for i in 0..zero_one_count.len() {
        // println!(
        //     "i: {}, 0: {}, 1: {} ",
        //     i, zero_one_count[i].0, zero_one_count[i].1
        // );

        let target_char: char;
        if zero_one_count[i].0 <= zero_one_count[i].1 {
            target_char = '0';
        } else {
            target_char = '1';
        }

        if result.len() == 0 {
            result = get_match_string(&lines, i, target_char);
        } else {
            result = get_match_string(&result, i, target_char);
        }
        zero_one_count = get_zero_one_count(&result);

        if result.len() == 1 {
            break;
        }
    }

    result[0].clone()
}
