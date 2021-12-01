use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_increase_count(number_list: &Vec<i32>) -> i32 {
    let _number_count = number_list.len();
    let mut _change_count = 0;
    for x in 1.._number_count {
        if number_list[x] > number_list[x - 1] {
            _change_count += 1;
        }
    }

    _change_count
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1
    let mut number_list: Vec<i32> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let current: i32 = line.unwrap().parse().unwrap();
        number_list.push(current);
    }

    println!("{}", get_increase_count(&number_list));

    // part 2
    let mut number_list_2: Vec<i32> = Vec::new();
    let number_count = number_list.len();
    for x in 0..number_count - 2 {
        number_list_2.push(number_list[x] + number_list[x + 1] + number_list[x + 2]);
    }

    println!("{}", get_increase_count(&number_list_2));
}
