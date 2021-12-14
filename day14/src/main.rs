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

    let mut mapping: BTreeMap<String, char> = BTreeMap::new();
    let mut template: Vec<char> = Vec::new();
    for line in contents.lines() {
        if line.contains("->") {
            let re = Regex::new(r"([A-Z]+) -> ([A-Z])").unwrap();
            for cap in re.captures_iter(&line) {
                let (pair, element) = (
                    &cap[1].parse::<String>().unwrap(),
                    &cap[2].parse::<char>().unwrap(),
                );
                mapping.insert(pair.to_string(), *element);
            }
        } else if line.len() > 1 {
            template = line.chars().collect::<Vec<char>>();
        }
    }

    // debug!("mapping {:?}", mapping);
    debug!("Template:     {:?}", String::from_iter(&template));

    for i in 0..10 {
        let result = replacement(&mapping, &template);
        debug!("After step {:?}: {:?}", i + 1, String::from_iter(&result));
        template = result.clone();
    }

    let frequencies = template.iter().fold(BTreeMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });
    debug!("{:?}", frequencies);

    let max = frequencies.values().max().unwrap();
    let min = frequencies.values().min().unwrap();
    info!("part 1: max {:?} - min {:?} = {:?}", max, min, max - min);
}

fn replacement(mapping: &BTreeMap<String, char>, template: &Vec<char>) -> Vec<char> {
    debug!("mapping {:?}", mapping);
    debug!("template {:?}", template);

    let mut result: Vec<char> = Vec::new();
    for (i, c) in template.iter().enumerate() {
        result.push(*c);

        if i == template.len() - 1 {
            break;
        }

        let pair = String::from_iter(template.iter().skip(i).take(2));
        result.push(mapping[&pair]);
    }

    debug!("{:?}", result);
    result
}
