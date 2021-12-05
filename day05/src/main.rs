use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut count_matrix_2: Vec<Vec<usize>> = vec![vec![0; y_max]; x_max];
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
                fill_h(&mut count_matrix_2, *x1, *x2, *y1);
            } else if x1 == x2 {
                fill_v(&mut count_matrix, *y1, *y2, *x1);
                fill_v(&mut count_matrix_2, *y1, *y2, *x1);
            } else if (*x1 as i32 - *x2 as i32).abs() == (*y1 as i32 - *y2 as i32).abs() {
                fill_d(&mut count_matrix_2, *x1, *x2, *y1, *y2);
            }
        }
    }

    let mut overlap_count: usize = 0;
    for i in 0..count_matrix.len() {
        for j in 0..count_matrix.len() {
            if count_matrix[i][j] > 1 {
                overlap_count += 1;
            }
        }
    }
    println!("part 1: {:?}", overlap_count);

    let mut overlap_count: usize = 0;
    for i in 0..count_matrix_2.len() {
        for j in 0..count_matrix_2.len() {
            if count_matrix_2[i][j] > 1 {
                overlap_count += 1;
            }
        }
    }
    println!("part 2: {:?}", overlap_count);
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

fn fill_d(
    count_matrix: &mut Vec<Vec<usize>>,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
) {
    // println!("d before {:?}", count_matrix);

    let x_multiplier: i32 = if x_start > x_end { -1 } else { 1 };
    let y_multiplier: i32 = if y_start > y_end { -1 } else { 1 };
    let diff: i32 = (x_start as i32 - x_end as i32).abs() + 1;

    for z in 0..diff {
        let i: usize = (x_start as i32 + (x_multiplier * z)) as usize;
        let j: usize = (y_start as i32 + (y_multiplier * z)) as usize;
        // println!("i {:?} j {:?}", i, j);
        count_matrix[j][i] += 1;
    }

    // println!("d after {:?}", count_matrix);
}
