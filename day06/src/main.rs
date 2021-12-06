use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1
    let mut fish_list: Vec<i32> = Vec::new();
    for line in reader.lines() {
        fish_list = line
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    println!("Initial state: {:?}", fish_list);
    let fish_list_2: Vec<i32> = fish_list.clone();

    after_timer(80, &mut fish_list);
    println!("part 1 (80): {}", fish_list.len());

    let result = optimize_timer(18, &fish_list_2);
    println!("part 2 (18): {}", result);

    let result = optimize_timer(80, &fish_list_2);
    println!("part 2 (80): {}", result);

    let result = optimize_timer(256, &fish_list_2);
    println!("part 2 (256): {}", result);
}

fn after_timer(days: i32, fish_list: &mut Vec<i32>) {
    for i in 0..days {
        let zero_count = fish_list.iter().filter(|x| **x == 0).count();
        // println!("zero: {}", zero_count);

        for j in 0..fish_list.len() {
            let tmp_time = if fish_list[j as usize] - 1 == -1 {
                6
            } else {
                fish_list[j as usize] - 1
            };

            fish_list[j as usize] = tmp_time;
        }

        for _ in 0..zero_count {
            fish_list.push(8);
        }

        // println!("After {:-2} day: {:?}", i + 1, fish_list);
    }
}

fn optimize_timer(days: i32, fish_list: &Vec<i32>) -> i128 {
    let mut counts = [0; 9];
    for fish in fish_list {
        counts[*fish as usize] += 1;
    }
    // println!("After {:-2} day: {:?}", 0, counts);

    for i in 0..days {
        let new_fish = counts[0];
        for j in 0..counts.len() - 1 {
            counts[j] = counts[j + 1];
        }
        counts[8] = new_fish;
        counts[6] += new_fish;
        // println!("After {:-2} day: {:?}", i + 1, counts);
    }

    counts.iter().sum()
}
