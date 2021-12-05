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
    let mut numbers: Vec<i32> = Vec::new();
    let mut value_matrix_list: Vec<[[i32; 5]; 5]> = Vec::new();
    let mut count_matrix_list: Vec<[[i32; 5]; 5]> = Vec::new();
    let mut matrix: [[i32; 5]; 5] = [[0; 5]; 5];
    let mut matrix_count = 0;

    for (count, line) in reader.lines().enumerate() {
        if count == 0 {
            numbers = line
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
        } else {
            let mut row_count = (count - 1) % 6;

            if row_count == 0 {
                matrix = [[0; 5]; 5];
                count_matrix_list.push(matrix);
            } else {
                let value_list: Vec<i32> = line
                    .unwrap()
                    .split(" ")
                    .filter(|x| !x.trim().is_empty())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

                for j in 0..value_list.len() {
                    matrix[row_count - 1][j] += value_list[j];
                }

                if row_count == 5 {
                    value_matrix_list.push(matrix);
                    matrix_count += 1;
                }
            }
        }
    }

    get_part1_final_answer(
        &numbers,
        &value_matrix_list,
        &count_matrix_list,
        matrix_count,
    );
    get_part2_final_answer(
        &numbers,
        &value_matrix_list,
        &count_matrix_list,
        matrix_count,
    );
}

fn mark(value_matrix: &[[i32; 5]; 5], count_matrix: &mut [[i32; 5]; 5], target: i32) {
    for i in 0..value_matrix.len() {
        for j in 0..value_matrix[0].len() {
            if value_matrix[i][j] == target {
                count_matrix[i][j] = 1;
            }
        }
    }

    // println!("target {:?}", target);
    // println!("value_matrix {:?}", value_matrix);
    // println!("count_matrix {:?}", count_matrix);
}

fn win(count_matrix: &[[i32; 5]; 5]) -> bool {
    let mut count: i32;
    for i in 0..count_matrix.len() {
        count = 0;
        for j in 0..count_matrix[0].len() {
            if count_matrix[i][j] == 1 {
                count += 1;
            }

            if count == 5 {
                return true;
            }
        }
    }

    for j in 0..count_matrix[0].len() {
        count = 0;
        for i in 0..count_matrix.len() {
            if count_matrix[i][j] == 1 {
                count += 1;
            }

            if count == 5 {
                return true;
            }
        }
    }

    false
}

fn get_marked_unmarked(
    value_matrix: &[[i32; 5]; 5],
    count_matrix: &[[i32; 5]; 5],
) -> (Vec<i32>, Vec<i32>) {
    let mut marked: Vec<i32> = Vec::new();
    let mut unmarked: Vec<i32> = Vec::new();

    for i in 0..value_matrix.len() {
        for j in 0..value_matrix[0].len() {
            if count_matrix[i][j] == 1 {
                marked.push(value_matrix[i][j]);
            } else {
                unmarked.push(value_matrix[i][j]);
            }
        }
    }

    (marked, unmarked)
}

fn get_part1_final_answer(
    numbers: &Vec<i32>,
    value_matrix_list: &Vec<[[i32; 5]; 5]>,
    source_count_matrix_list: &Vec<[[i32; 5]; 5]>,
    matrix_count: i32,
) {
    let mut count_matrix_list = source_count_matrix_list.clone();

    let mut win_count: Vec<usize> = Vec::new();
    let mut final_drawn: i32 = -1;
    let mut win_matrix_num: usize;

    for drawn in numbers {
        for i in 0..matrix_count {
            mark(
                &value_matrix_list[i as usize],
                &mut count_matrix_list[i as usize],
                *drawn,
            );

            if win(&count_matrix_list[i as usize]) {
                final_drawn = *drawn;
                win_matrix_num = i as usize;

                if !win_count.contains(&win_matrix_num) {
                    win_count.push(win_matrix_num);
                }

                break;
            }
        }

        if win_count.len() == 1 {
            break;
        }
    }

    win_matrix_num = *win_count.last().unwrap();
    println!("final_drawn {:?}", final_drawn);
    println!("win_matrix_num {:?}", win_matrix_num);
    println!(
        "value_matrix {:?}",
        value_matrix_list[win_matrix_num as usize]
    );
    println!(
        "count_matrix {:?}",
        count_matrix_list[win_matrix_num as usize]
    );

    let (marked, unmarked) = get_marked_unmarked(
        &value_matrix_list[win_matrix_num as usize],
        &count_matrix_list[win_matrix_num as usize],
    );
    println!("marked {:?}", marked);
    println!("unmarked {:?}", unmarked);

    let final_sum = unmarked.iter().sum::<i32>();
    println!("final_sum {:?}", final_sum);

    println!(
        "part 1: {} * {} = {} \n",
        final_drawn,
        final_sum,
        final_drawn * final_sum
    );
}

fn get_part2_final_answer(
    numbers: &Vec<i32>,
    value_matrix_list: &Vec<[[i32; 5]; 5]>,
    source_count_matrix_list: &Vec<[[i32; 5]; 5]>,
    matrix_count: i32,
) {
    let mut count_matrix_list = source_count_matrix_list.clone();

    let mut win_count: Vec<usize> = Vec::new();
    let mut final_drawn: i32 = -1;
    let mut win_matrix_num: usize;

    for drawn in numbers {
        for i in 0..matrix_count {
            mark(
                &value_matrix_list[i as usize],
                &mut count_matrix_list[i as usize],
                *drawn,
            );

            if win(&count_matrix_list[i as usize]) {
                final_drawn = *drawn;
                win_matrix_num = i as usize;

                if !win_count.contains(&win_matrix_num) {
                    win_count.push(win_matrix_num);
                }
            }
        }

        if win_count.len() == matrix_count as usize {
            break;
        }
    }

    win_matrix_num = *win_count.last().unwrap();
    println!("final_drawn {:?}", final_drawn);
    println!("win_matrix_num {:?}", win_matrix_num);
    println!(
        "value_matrix {:?}",
        value_matrix_list[win_matrix_num as usize]
    );
    println!(
        "count_matrix {:?}",
        count_matrix_list[win_matrix_num as usize]
    );

    let (marked, unmarked) = get_marked_unmarked(
        &value_matrix_list[win_matrix_num as usize],
        &count_matrix_list[win_matrix_num as usize],
    );
    println!("marked {:?}", marked);
    println!("unmarked {:?}", unmarked);

    let final_sum = unmarked.iter().sum::<i32>();
    println!("final_sum {:?}", final_sum);

    println!(
        "part 2: {} * {} = {} \n",
        final_drawn,
        final_sum,
        final_drawn * final_sum
    );
}
