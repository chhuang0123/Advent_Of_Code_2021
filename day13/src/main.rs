use log::{debug, info};
use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Point(usize, usize);

#[derive(Debug)]
struct Fold(char, usize);

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut fold_list: Vec<Fold> = Vec::new();
    let mut point_list: Vec<Point> = Vec::new();
    let mut x_max = 0;
    let mut y_max = 0;

    for line in contents.lines() {
        debug!("{:?}", line);

        if line.contains(",") {
            let xy: Vec<&str> = line.split(",").collect();
            let x = xy[0].parse::<usize>().unwrap();
            let y = xy[1].parse::<usize>().unwrap();

            x_max = if x > x_max { x } else { x_max };
            y_max = if y > x_max { y } else { y_max };
            debug!("x_max {:?} y_max {:?}", x_max, y_max);

            debug!("x {:?} y {:?}", x, y);
            point_list.push(Point(x, y));
        } else if line.contains("=") {
            let re = Regex::new(r"([xy])=(\d+)").unwrap();
            for cap in re.captures_iter(&line) {
                let (axis, value) = (
                    &cap[1].parse::<char>().unwrap(),
                    &cap[2].parse::<usize>().unwrap(),
                );

                debug!("axis {:?} value {:?}", axis, value);
                fold_list.push(Fold(*axis, *value));
            }
        }
    }

    debug!("x_max {:?} y_max {:?}", x_max, y_max);
    debug!("point_list {:?}", point_list);
    debug!("fold_list {:?}", fold_list);

    let mut point_matrix = vec![vec!['.'; x_max + 1]; y_max + 1];
    debug!("point_matrix {:?}", point_matrix);
    fill(&point_list, &mut point_matrix);

    fold(&fold_list, &mut point_matrix)
}

fn get_point(point_matrix: &Vec<Vec<char>>) -> Vec<Point> {
    let y_max = point_matrix.len();
    let x_max = point_matrix[0].len();
    let mut point_list: Vec<Point> = Vec::new();

    for x in 0..x_max {
        for y in 0..y_max {
            if point_matrix[y][x] == '#' {
                point_list.push(Point(x, y));
            }
        }
    }

    point_list
}

fn fill(point_list: &Vec<Point>, point_matrix: &mut Vec<Vec<char>>) {
    debug!("point_matrix - before fill {:?}", point_matrix);
    for point in point_list {
        point_matrix[point.1][point.0] = '#';
    }
    debug!("point_matrix - after fill {:?}", point_matrix);
}

fn get_dot_count(x_max: usize, y_max: usize, point_matrix: &Vec<Vec<char>>) -> usize {
    let mut count: usize = 0;
    for x in 0..x_max {
        for y in 0..y_max {
            if point_matrix[y][x] == '#' {
                count += 1;
            }
        }
    }

    count
}

fn fold(fold_list: &Vec<Fold>, point_matrix: &mut Vec<Vec<char>>) {
    let mut y_max = point_matrix.len();
    let mut x_max = point_matrix[0].len();
    let mut fold_count = 0;

    for fold in fold_list {
        debug!("point_matrix before fold {:?}", point_matrix);
        let point_list: Vec<Point> = get_point(point_matrix);
        debug!("point_list {:?}", point_list);

        for point in &point_list {
            if fold.0 == 'y' {
                y_max = fold.1;
                if point.1 > fold.1 {
                    // debug!("before {:?} {:?}", point.0, point.1);
                    let new_y = (fold.1 - (point.1 % fold.1)) % fold.1;
                    // debug!("after {:?} {:?}", point.0, new_y);
                    point_matrix[new_y][point.0] = '#';
                }
            } else {
                x_max = fold.1;
                if point.0 > fold.1 {
                    // debug!("before {:?} {:?}", point.0, point.1);
                    let new_x = (fold.1 - (point.0 % fold.1)) % fold.1;
                    // debug!("after {:?} {:?}", new_x, point.1);
                    point_matrix[point.1][new_x] = '#';
                }
            }
        }

        fold_count += 1;
        let dot_count = get_dot_count(x_max, y_max, point_matrix);
        debug!(
            "point_matrix after fold {} time {:?} --{:?}--",
            fold_count, point_matrix, dot_count
        );

        if fold_count == 1 {
            info!("part 1: {}", dot_count);
        }
    }
}
