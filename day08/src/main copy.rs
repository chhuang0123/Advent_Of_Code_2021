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
                debug!("1 {:?}", output_value);
                // let value_count = output_value
                //     .split(" ")
                //     .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                //     .count();
                // digits_sum += value_count;
                // debug!("{:?} {:?}", value_count, digits_sum);
            } else {
                debug!("0 {:?}", output_value);
                let digial_map = generate_map(output_value);
                debug!("{:?}\n->{:?}", output_value, digial_map);
            }
        }
    }
    // info!("part 1: {:?}", digits_sum);
}

fn generate_map(digitals: &str) -> HashMap<i32, HashSet<char>> {
    debug!("generate_map {:?}", digitals);

    let mut digital_map: HashMap<i32, HashSet<char>> = HashMap::new();
    let digital_list: Vec<&str> = digitals.split(" ").collect();

    let mut length_set_6: Vec<HashSet<char>> = Vec::new();
    let mut length_set_5: Vec<HashSet<char>> = Vec::new();

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
    debug!("digital_map {:?}", digital_map);

    let diff_8_4_7: HashSet<char> = (digital_map[&8]
        .difference(&digital_map[&4])
        .cloned()
        .collect::<HashSet<char>>())
    .difference(&digital_map[&7])
    .cloned()
    .collect::<HashSet<char>>();
    debug!("diff_8_4_7 {:?}", diff_8_4_7);

    let digital_list: Vec<&str> = digitals.split(" ").collect();
    for digital in digital_list {
        debug!("digital_map {:?}", digital_map);
        debug!("digital {:?} ({:?})", digital, digital.len());

        let set_2: HashSet<char> = digital_map[&1].clone();
        if digital.len() == 6 {
            debug!("666666 digital {:?}", digital);
            let set_len_6 = digital.chars().collect::<HashSet<char>>();
            if set_len_6
                .intersection(&diff_8_4_7)
                .cloned()
                .collect::<HashSet<char>>()
                .len()
                == 1
            {
                println!("111");
                digital_map.insert(9, digital.chars().collect::<HashSet<char>>());
                debug!("9999 digital_map {:?}", digital_map);
            }

            let len = set_len_6
                .intersection(&set_2)
                .cloned()
                .collect::<HashSet<char>>()
                .len();
            if len == 1 {
                println!("22-11");
                digital_map.insert(6, digital.chars().collect::<HashSet<char>>());
            } else if len == 2 {
                println!("22-22");
                digital_map.insert(0, digital.chars().collect::<HashSet<char>>());
            }
        } else if digital.len() == 5 {
            let set_len_5 = digital.chars().collect::<HashSet<char>>();
            if set_len_5
                .intersection(&set_2)
                .cloned()
                .collect::<HashSet<char>>()
                .len()
                == 2
            {
                digital_map.insert(3, digital.chars().collect::<HashSet<char>>());
                continue;
            }

            let len = set_len_5
                .intersection(&digital_map[&6])
                .cloned()
                .collect::<HashSet<char>>()
                .len();

            if len == 5 {
                digital_map.insert(5, digital.chars().collect::<HashSet<char>>());
                continue;
            } else if len == 4 {
                digital_map.insert(2, digital.chars().collect::<HashSet<char>>());
                continue;
            }
        }
    }

    debug!("digital_map {:#?}", digital_map);
    digital_map
}
