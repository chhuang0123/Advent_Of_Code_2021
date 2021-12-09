use log::{debug, info};
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
        debug!("{:?}", numbers);

        numbers_of_numbers.push(numbers);
    }

    let row_count = numbers_of_numbers.len();
    let column_count = numbers_of_numbers[0].len();
    debug!(
        "row: {} column: {}\n{:?} ",
        row_count, column_count, numbers_of_numbers
    );

    let mut value_list: Vec<u8> = Vec::new();
    let mut low_points: Vec<Point> = Vec::new();
    for i in 0..row_count {
        for j in 0..column_count {
            debug!("i {} j {} {:?}", i, j, numbers_of_numbers[i][j]);
            if check_up(i, j, &numbers_of_numbers)
                && check_down(i, j, &numbers_of_numbers)
                && check_left(i, j, &numbers_of_numbers)
                && check_right(i, j, &numbers_of_numbers)
            {
                debug!("push {}", numbers_of_numbers[i][j]);
                value_list.push(numbers_of_numbers[i][j]);
                low_points.push(Point(i, j));
            }
        }
    }
    debug!("value_list {:?}", value_list);
    info!(
        "part 1: {:?}",
        value_list.iter().map(|x| (*x as u32) + 1).sum::<u32>()
    );

    debug!("low_points {:?}", low_points);

    let mut basin_points_counts: Vec<usize> = Vec::new();
    for low_point in low_points {
        debug!("low_point {:?}", low_point);
        let basin_points = find_basin(&low_point, &numbers_of_numbers);
        debug!("basin_points ({}) {:?} ", basin_points.len(), basin_points);
        basin_points_counts.push(basin_points.len());
    }

    basin_points_counts.sort();
    basin_points_counts.reverse();
    debug!("basin_points_counts {:?} ", basin_points_counts);
    info!(
        "part 2: {:?} ",
        basin_points_counts[0] * basin_points_counts[1] * basin_points_counts[2]
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

fn find_up(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> Option<Point> {
    if i == 0 {
        return None;
    }

    let actual_value = numbers_of_numbers[i][j];
    let up_value = numbers_of_numbers[i - 1][j];

    debug!(
        "find_up i {} j {} {:?}",
        i - 1,
        j,
        numbers_of_numbers[i - 1][j]
    );

    if up_value > actual_value && up_value != 9 {
        return Some(Point(i - 1, j));
    } else {
        return None;
    }
}

fn find_down(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> Option<Point> {
    let row_count = numbers_of_numbers.len();
    if i == row_count - 1 {
        return None;
    }

    let actual_value = numbers_of_numbers[i][j];
    let down_value = numbers_of_numbers[i + 1][j];

    debug!(
        "find_down i {} j {} {:?}",
        i + 1,
        j,
        numbers_of_numbers[i + 1][j]
    );

    if down_value > actual_value && down_value != 9 {
        return Some(Point(i + 1, j));
    } else {
        return None;
    }
}

fn find_left(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> Option<Point> {
    if j == 0 {
        return None;
    }

    let actual_value = numbers_of_numbers[i][j];
    let left_value = numbers_of_numbers[i][j - 1];

    debug!(
        "find_left i {} j {} {:?}",
        i,
        j - 1,
        numbers_of_numbers[i][j - 1]
    );

    if left_value > actual_value && left_value != 9 {
        return Some(Point(i, j - 1));
    } else {
        return None;
    }
}

fn find_right(i: usize, j: usize, numbers_of_numbers: &Vec<Vec<u8>>) -> Option<Point> {
    let column_count = numbers_of_numbers[0].len();
    if j == column_count - 1 {
        return None;
    }

    let actual_value = numbers_of_numbers[i][j];
    let right_value = numbers_of_numbers[i][j + 1];

    debug!(
        "find_right i {} j {} {:?}",
        i,
        j + 1,
        numbers_of_numbers[i][j + 1]
    );

    if right_value > actual_value && right_value != 9 {
        return Some(Point(i, j + 1));
    } else {
        return None;
    }
}

fn find_basin(low_point: &Point, numbers_of_numbers: &Vec<Vec<u8>>) -> HashSet<Point> {
    let mut check_points: HashSet<Point> = HashSet::new();
    let mut basin_points: HashSet<Point> = HashSet::new();
    basin_points.insert((*low_point).clone());

    loop {
        for check_point in basin_points.clone() {

            if check_points.contains(&check_point) {
                continue;
            }

            if let Some(basin_point) = find_up(check_point.0, check_point.1, &numbers_of_numbers) {
                debug!("basin_point {:?}", basin_point);
                basin_points.insert(basin_point);
            }

            if let Some(basin_point) = find_down(check_point.0, check_point.1, &numbers_of_numbers)
            {
                debug!("basin_point {:?}", basin_point);
                basin_points.insert(basin_point);
            }

            if let Some(basin_point) = find_left(check_point.0, check_point.1, &numbers_of_numbers)
            {
                debug!("basin_point {:?}", basin_point);
                basin_points.insert(basin_point);
            }

            if let Some(basin_point) = find_right(check_point.0, check_point.1, &numbers_of_numbers)
            {
                debug!("basin_point {:?}", basin_point);
                basin_points.insert(basin_point);
            }

            check_points.insert(check_point);
        }

        debug!(
            "check_points {:?} ({}) basin_points {:?} ({})",
            check_points,
            check_points.len(),
            basin_points,
            basin_points.len()
        );
        if check_points.len() == basin_points.len() {
            return basin_points;
        }
    }
}
