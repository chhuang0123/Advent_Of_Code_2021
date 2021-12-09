use log::{debug, info};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut numbers_of_numbers: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        debug!("{:?}", line);

        let numbers: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        debug!("{:?}", numbers);

        numbers_of_numbers.push(numbers);
    }

    let row_count = numbers_of_numbers.len();
    let column_count = numbers_of_numbers[0].len();
    debug!(
        "row: {} column: {}\n{:?} ",
        row_count, column_count, numbers_of_numbers
    );

    let mut result: Vec<u8> = Vec::new();
    for i in 0..row_count {
        for j in 0..column_count {
            debug!("i {} j {} {:?}", i, j, numbers_of_numbers[i][j]);
            if check_up(i, j, &numbers_of_numbers)
                && check_down(i, j, &numbers_of_numbers)
                && check_left(i, j, &numbers_of_numbers)
                && check_right(i, j, &numbers_of_numbers)
            {
                debug!("push {}", numbers_of_numbers[i][j]);
                result.push(numbers_of_numbers[i][j]);
            }
        }
    }
    debug!("result {:?}", result);
    info!(
        "part 1: {:?}",
        result.iter().map(|x| (*x as u32) + 1).sum::<u32>()
    );
}

fn check_up(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> bool {
    if i == 0 {
        return true;
    }

    let actual_value = numbers_of_numbers[i][j];
    let up_value = numbers_of_numbers[i - 1][j];

    debug!(
        "check_up i {} j {} {:?}",
        i - 1,
        j,
        numbers_of_numbers[i - 1][j]
    );

    if actual_value < up_value {
        return true;
    } else {
        return false;
    }
}

fn check_down(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> bool {
    let row_count = numbers_of_numbers.len();
    if i == row_count - 1 {
        return true;
    }

    let actual_value = numbers_of_numbers[i][j];
    let down_value = numbers_of_numbers[i + 1][j];

    debug!(
        "check_down i {} j {} {:?}",
        i + 1,
        j,
        numbers_of_numbers[i + 1][j]
    );

    if actual_value < down_value {
        return true;
    } else {
        return false;
    }
}

fn check_left(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> bool {
    if j == 0 {
        return true;
    }

    let actual_value = numbers_of_numbers[i][j];
    let left_value = numbers_of_numbers[i][j - 1];

    debug!(
        "check_left i {} j {} {:?}",
        i,
        j - 1,
        numbers_of_numbers[i][j - 1]
    );

    if actual_value < left_value {
        return true;
    } else {
        return false;
    }
}

fn check_right(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> bool {
    let column_count = numbers_of_numbers[0].len();
    if j == column_count - 1 {
        return true;
    }

    let actual_value = numbers_of_numbers[i][j];
    let right_value = numbers_of_numbers[i][j + 1];

    debug!(
        "check_right i {} j {} {:?}",
        i,
        j + 1,
        numbers_of_numbers[i][j + 1]
    );

    if actual_value < right_value {
        return true;
    } else {
        return false;
    }
}
