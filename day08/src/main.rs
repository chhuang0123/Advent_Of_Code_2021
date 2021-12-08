use log::{debug, info};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    // debug!("{}", filename);

    let mut digits_sum = 0;
    for line in contents.lines() {
        let patterns_value_list: Vec<&str> = line.split(" | ").collect();
        for (index, output_value) in patterns_value_list.iter().enumerate() {
            if index % 2 == 1 {
                info!("1 {:?}", output_value);
                let value_count = output_value
                    .split(" ")
                    .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                    .count();
                digits_sum += value_count;
                debug!("{:?} {:?}", value_count, digits_sum);
            } else {
                info!("0 {:?}", output_value);
                let digital_map = generate_map(output_value);
                info!("{:?}\n->{:?} ({:?})", output_value, digital_map, digital_map.len());
            }
        }
    }
    info!("part 1: {:?}", digits_sum);
}

fn generate_map(digitals: &str) -> HashMap<i32, HashSet<char>> {
    debug!("generate_map {:?}", digitals);

    let mut digital_map: HashMap<i32, HashSet<char>> = HashMap::new();
    let digital_list: Vec<&str> = digitals.split(" ").collect();

    let mut length_set_5: Vec<HashSet<char>> = Vec::new();
    let mut length_set_6: Vec<HashSet<char>> = Vec::new();

    for digital in digital_list {
        if digital.len() == 2 {
            digital_map.insert(1, digital.chars().collect::<HashSet<char>>());
        } else if digital.len() == 3 {
            digital_map.insert(7, digital.chars().collect::<HashSet<char>>());
        } else if digital.len() == 4 {
            digital_map.insert(4, digital.chars().collect::<HashSet<char>>());
        } else if digital.len() == 7 {
            digital_map.insert(8, digital.chars().collect::<HashSet<char>>());
        } else if digital.len() == 5 {
            length_set_5.push(digital.chars().collect::<HashSet<char>>());
        } else if digital.len() == 6 {
            length_set_6.push(digital.chars().collect::<HashSet<char>>());
        }
    }

    let set_2: HashSet<char> = digital_map[&1].clone();
    debug!("set_2 {:?}", set_2);

    let diff_8_4_7: HashSet<char> = (digital_map[&8]
        .difference(&digital_map[&4])
        .cloned()
        .collect::<HashSet<char>>())
    .difference(&digital_map[&7])
    .cloned()
    .collect::<HashSet<char>>();
    debug!("diff_8_4_7 {:?}", diff_8_4_7);

    for tmp_set in length_set_6.clone() {
        let intersection_count = tmp_set
            .intersection(&diff_8_4_7)
            .cloned()
            .collect::<HashSet<char>>()
            .len();

        if intersection_count == 1 {
            digital_map.insert(9, tmp_set);
        } else if intersection_count == 2 {
            let intersection_count = tmp_set
                .intersection(&set_2)
                .cloned()
                .collect::<HashSet<char>>()
                .len();
            if intersection_count == 1 {
                digital_map.insert(6, tmp_set);
            } else if intersection_count == 2 {
                digital_map.insert(0, tmp_set);
            }
        }
    }

    let set_6: HashSet<char> = digital_map[&6].clone();

    for tmp_set in length_set_5.clone() {
        let intersection_count = tmp_set
            .intersection(&set_2)
            .cloned()
            .collect::<HashSet<char>>()
            .len();

        if intersection_count == 2 {
            digital_map.insert(3, tmp_set);
        } else if intersection_count == 1 {
            let intersection_count = tmp_set
                .intersection(&set_6)
                .cloned()
                .collect::<HashSet<char>>()
                .len();
            if intersection_count == 5 {
                digital_map.insert(5, tmp_set);
            } else if intersection_count == 4 {
                digital_map.insert(2, tmp_set);
            }
        }
    }

    debug!("digital_map {:?}", digital_map);
    debug!("length_set_5 {:?}", length_set_5);
    debug!("length_set_6 {:?}", length_set_6);
    digital_map
}
