use log::{debug, info};
use std::env;
use std::fs;

#[derive(Debug)]
struct Point(usize, usize);

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
        numbers_of_numbers.push(numbers);
    }
    debug!("{:?}", numbers_of_numbers);

    let mut flash_count = 0;
    let mut count = 1;
    loop {
        let result = change_energy_level(&mut numbers_of_numbers);
        flash_count += result.0;
        debug!("step: {} {:?}", count, numbers_of_numbers);
        debug!(
            "{:?}  -> result {} sum {}",
            numbers_of_numbers, result.0, flash_count
        );

        if count == 100 {
            info!("part 1: {:?}", flash_count);
        }

        if result.1 {
            info!("part 2: {:?}", count);
            break;
        }

        count += 1;
    }
}

fn change_energy_level(numbers_of_numbers: &mut Vec<Vec<u8>>) -> (u32, bool) {
    let row_count = numbers_of_numbers.len();
    let column_count = numbers_of_numbers[0].len();

    // add 1
    let mut greater_10_points: Vec<Point> = Vec::new();
    for i in 0..row_count {
        for j in 0..column_count {
            numbers_of_numbers[i][j] += 1;
            if numbers_of_numbers[i][j] == 10 {
                greater_10_points.push(Point(i, j));
            }
        }
    }
    debug!("greater_10_points {:?}", greater_10_points);

    // flash
    flash(&mut greater_10_points, &mut *numbers_of_numbers);

    // reset
    let mut flash_count: u32 = 0;
    let mut zero_count: u32 = 0;

    for i in 0..row_count {
        for j in 0..column_count {
            if numbers_of_numbers[i][j] > 9 {
                numbers_of_numbers[i][j] = 0;
                flash_count += 1;
            }
            if numbers_of_numbers[i][j] == 0 {
                zero_count += 1;
            }
        }
    }

    (flash_count, row_count * column_count == zero_count as usize)
}

fn flash(greater_10_points: &mut Vec<Point>, numbers_of_numbers: &mut Vec<Vec<u8>>) {
    // debug!("flash - {:?}", numbers_of_numbers);
    let row_count = numbers_of_numbers.len();
    let column_count = numbers_of_numbers[0].len();

    while greater_10_points.len() > 0 {
        let point = greater_10_points.pop().unwrap();
        let i = point.0;
        let j = point.1;

        debug!("-- {} {}", i, j);

        // up
        if i != 0 {
            debug!("{} {}", i - 1, j);
            numbers_of_numbers[i - 1][j] += 1;
            if numbers_of_numbers[i - 1][j] == 10 {
                greater_10_points.push(Point(i - 1, j));
            }
        }

        // down
        if i != row_count - 1 {
            debug!("{} {}", i + 1, j);
            numbers_of_numbers[i + 1][j] += 1;
            if numbers_of_numbers[i + 1][j] == 10 {
                greater_10_points.push(Point(i + 1, j));
            }
        }

        // left
        if j != 0 {
            debug!("{} {}", i, j - 1);
            numbers_of_numbers[i][j - 1] += 1;
            if numbers_of_numbers[i][j - 1] == 10 {
                greater_10_points.push(Point(i, j - 1));
            }
        }

        // down
        if j != column_count - 1 {
            debug!("{} {}", i, j + 1);
            numbers_of_numbers[i][j + 1] += 1;
            if numbers_of_numbers[i][j + 1] == 10 {
                greater_10_points.push(Point(i, j + 1));
            }
        }

        // left-up
        if i != 0 && j != 0 {
            debug!("{} {}", i - 1, j - 1);
            numbers_of_numbers[i - 1][j - 1] += 1;
            if numbers_of_numbers[i - 1][j - 1] == 10 {
                greater_10_points.push(Point(i - 1, j - 1));
            }
        }

        // right-up
        if i != 0 && j != column_count - 1 {
            debug!("{} {}", i - 1, j + 1);
            numbers_of_numbers[i - 1][j + 1] += 1;
            if numbers_of_numbers[i - 1][j + 1] == 10 {
                greater_10_points.push(Point(i - 1, j + 1));
            }
        }

        // left-down
        if i != row_count - 1 && j != 0 {
            debug!("{} {}", i + 1, j - 1);
            numbers_of_numbers[i + 1][j - 1] += 1;
            if numbers_of_numbers[i + 1][j - 1] == 10 {
                greater_10_points.push(Point(i + 1, j - 1));
            }
        }

        // right-down
        if i != row_count - 1 && j != column_count - 1 {
            debug!("{} {}", i + 1, j + 1);
            numbers_of_numbers[i + 1][j + 1] += 1;
            if numbers_of_numbers[i + 1][j + 1] == 10 {
                greater_10_points.push(Point(i + 1, j + 1));
            }
        }
    }
}
