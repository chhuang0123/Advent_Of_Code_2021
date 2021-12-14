use log::{debug, info};
use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("filename {}", filename);

    let mut mapping: BTreeMap<(char, char), char> = BTreeMap::new();
    let mut template: Vec<char> = Vec::new();
    for line in contents.lines() {
        if line.contains("->") {
            let re = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").unwrap();
            for cap in re.captures_iter(&line) {
                let (c1, c2, element) = (
                    &cap[1].parse::<char>().unwrap(),
                    &cap[2].parse::<char>().unwrap(),
                    &cap[3].parse::<char>().unwrap(),
                );
                mapping.insert((*c1, *c2), *element);
            }
        } else if line.len() > 1 {
            template = line.chars().collect::<Vec<char>>();
        }
    }

    debug!("mapping {:?}", mapping);
    debug!("template {:?}", template);
    part1(&mapping, &template, 10);
    part2(&mapping, &template, 40);
}

fn part2(mapping: &BTreeMap<(char, char), char>, template: &Vec<char>, stop: usize) {
    let mut steps: BTreeMap<(char, char), usize> = BTreeMap::new();
    for (i, _) in template.iter().enumerate() {
        let pair = if i == template.len() - 1 {
            (template[i], ' ')
        } else {
            (template[i], template[i + 1])
        };
        steps.insert(pair, 1);
    }
    debug!("{:?}", steps);

    for i in 0..stop {
        let mut new_steps: BTreeMap<(char, char), usize> = BTreeMap::new();
        for (pair, value) in steps.iter() {
            if mapping.contains_key(pair) {
                let new_char = mapping[pair];
                debug!("new_step - {:?} {:?}", pair, new_char);
                debug!("new_step - 1 {:?}", (pair.0, new_char));
                debug!("new_step - 2 {:?}", (new_char, pair.1));
                *new_steps.entry((pair.0, new_char)).or_insert(0) += *value;
                *new_steps.entry((new_char, pair.1)).or_insert(0) += *value;
            } else {
                new_steps.insert(*pair, *value);
            }
        }

        debug!(
            "new_steps {:?} ({:?}) - {:?}",
            new_steps,
            new_steps.len(),
            new_steps.values().cloned().sum::<usize>()
        );
        steps = new_steps.clone();

        if i == stop - 1 {
            let mut final_result: BTreeMap<char, usize> = BTreeMap::new();
            for (pair, value) in steps.iter() {
                *final_result.entry((*pair).0).or_insert(0) += value;
            }

            let max = final_result.values().max().unwrap();
            let min = final_result.values().min().unwrap();
            info!("{:?}", final_result);
            info!("part 2: max {:?} - min {:?} = {:?}", max, min, max - min);
        }
    }
}

fn part1(mapping: &BTreeMap<(char, char), char>, template: &Vec<char>, stop: usize) {
    let mut template = template.clone();

    for i in 0..stop {
        let result = replacement(&mapping, &template);
        debug!("After step {:?}: {:?}", i + 1, String::from_iter(&result));
        template = result;
    }

    let frequencies = template.iter().fold(BTreeMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });
    info!("{:?}", frequencies);

    let max = frequencies.values().max().unwrap();
    let min = frequencies.values().min().unwrap();
    info!("part 1: max {:?} - min {:?} = {:?}", max, min, max - min);
}

fn replacement(mapping: &BTreeMap<(char, char), char>, template: &Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for (i, c) in template.iter().enumerate() {
        result.push(*c);

        if i == template.len() - 1 {
            break;
        }

        let pair = (template[i], template[i + 1]);
        result.push(mapping[&pair]);
    }

    debug!("result {:?}", result);
    result
}
