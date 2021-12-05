use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    for line in reader.lines() {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        for cap in re.captures_iter(&line.unwrap()) {
            let (x1, y1, x2, y2) = (
                &cap[1].parse::<usize>().unwrap(),
                &cap[2].parse::<usize>().unwrap(),
                &cap[3].parse::<usize>().unwrap(),
                &cap[4].parse::<usize>().unwrap(),
            );

            // println!("{:?} {:?} {:?} {:?}, {:?} {:?}", *x1, *y1, *x2, *y2, x_max, y_max);

            x_max = if *x1 > x_max { *x1 } else { x_max };
            x_max = if *x2 > x_max { *x2 } else { x_max };
            y_max = if *y1 > y_max { *y1 } else { y_max };
            y_max = if *y2 > y_max { *y2 } else { y_max };
        }
    }
    x_max += 1;
    y_max += 1;
    // println!("{:?} {:?}", x_max, y_max);

    let mut count_matrix: Vec<Vec<usize>> = vec![vec![0; y_max]; x_max];
    // println!("{:?}", count_matrix);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        for cap in re.captures_iter(&line.unwrap()) {
            let (x1, y1, x2, y2) = (
                &cap[1].parse::<usize>().unwrap(),
                &cap[2].parse::<usize>().unwrap(),
                &cap[3].parse::<usize>().unwrap(),
                &cap[4].parse::<usize>().unwrap(),
            );
            // println!("{:?} {:?} {:?} {:?}", *x1, *y1, *x2, *y2);

            if y1 == y2 {
                fill_h(&mut count_matrix, *x1, *x2, *y1);
            } else if x1 == x2 {
                fill_v(&mut count_matrix, *y1, *y2, *x1);
            }
        }
    }

    let mut overlap_count: usize = 0;
    for i in 0..count_matrix.len() {
        for j in 0..count_matrix.len() {
            if count_matrix[i][j] > 1{
                overlap_count += 1;
            }
        }
    }
    println!("part 1: {:?}", overlap_count);
}

fn fill_h(count_matrix: &mut Vec<Vec<usize>>, start: usize, end: usize, j: usize) {
    // println!("h before {:?}", count_matrix);

    let start_index = if start > end { end } else { start };
    let end_index = if end > start { end } else { start };

    for i in start_index..end_index + 1 {
        // println!("{:?} {:?} +1", i, j);
        count_matrix[j][i] += 1;
    }

    // println!("h after {:?}", count_matrix);
}

fn fill_v(count_matrix: &mut Vec<Vec<usize>>, start: usize, end: usize, i: usize) {
    // println!("v before {:?}", count_matrix);

    let start_index = if start > end { end } else { start };
    let end_index = if end > start { end } else { start };

    for j in start_index..end_index + 1 {
        // println!("{:?} {:?} +1", i, j);
        count_matrix[j][i] += 1;
    }

    // println!("v after {:?}", count_matrix);
}
