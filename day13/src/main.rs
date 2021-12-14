use log::{debug, info};
use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Point(usize, usize);

#[derive(Debug, Clone, Copy)]
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
            point_list.push(Point(y, x));
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

    let mut min_point = Point(0, 0);
    for (index, fold_axis) in fold_list.iter().enumerate() {
        debug!("index {:?} fold_axis {:?}", index, fold_axis);

        // print_all(&point_matrix);
        debug!("x_min {:?} y_min {:?}", min_point.0, min_point.1);
        fold(&mut min_point, *fold_axis, &mut point_matrix);
        debug!("x_min {:?} y_min {:?}", min_point.0, min_point.1);
        // print_all(&point_matrix);

        if index == 0 {
            let result = print_word(min_point.0, min_point.1, &point_matrix);
            info!("part 1: {}", result);
        }
    }

    // info!("x_min {:?} y_min {:?}", min_point.0, min_point.1);
    // print_word(min_point.0, min_point.1, &point_matrix);
}

fn print_word(x_max: usize, y_max: usize, point_matrix: &Vec<Vec<char>>) -> usize {
    let x_max = if x_max == 0 {
        point_matrix[0].len()
    } else {
        x_max
    };
    let y_max = if y_max == 0 {
        point_matrix.len()
    } else {
        y_max
    };

    let mut count = 0;
    for i in 0..y_max {
        for j in 0..x_max {
            print!("{}", point_matrix[i][j]);
            if point_matrix[i][j] == '#' {
                count += 1;
            }
        }
        println!("");
    }

    count
}

fn get_point(point_matrix: &Vec<Vec<char>>) -> Vec<Point> {
    let row_count = point_matrix.len();
    let column_count = point_matrix[0].len();
    debug!("row_count {:?} column_count {:?}", row_count, column_count);

    let mut point_list: Vec<Point> = Vec::new();
    for x in 0..row_count {
        for y in 0..column_count {
            if point_matrix[x][y] == '#' {
                point_list.push(Point(y, x));
            }
        }
    }

    point_list
}

fn fill(point_list: &Vec<Point>, point_matrix: &mut Vec<Vec<char>>) {
    debug!("before fill {:?}", point_matrix);
    for point in point_list {
        point_matrix[point.0][point.1] = '#';
    }
    debug!("after fill {:?}", point_matrix);
}

fn fold(min_point: &mut Point, fold: Fold, point_matrix: &mut Vec<Vec<char>>) {
    let row_count = point_matrix.len();
    let column_count = point_matrix[0].len();
    debug!("row_count {:?} column_count {:?}", row_count, column_count);

    debug!("min_point {:?}", min_point);
    let point_list: Vec<Point> = get_point(&point_matrix);
    debug!("point_list {:?}", point_list);

    let mut fold_point_lit: Vec<Point> = Vec::new();
    if fold.0 == 'y' {
        min_point.1 = fold.1;
        for point in &point_list {
            if point.1 > fold.1 && point.0 < row_count {
                debug!("y before {:?} {:?}", point.0, point.1);
                let new_y = (fold.1 - (point.1 % fold.1)) % fold.1;
                debug!(" - y after {:?} {:?}", point.0, new_y);
                fold_point_lit.push(Point(point.0, new_y));
            }
        }
    } else {
        min_point.0 = fold.1;
        for point in &point_list {
            if point.0 > fold.1 && point.1 < column_count {
                debug!("x before {:?} {:?}", point.0, point.1);
                let new_x = (fold.1 - (point.0 % fold.1)) % fold.1;
                debug!(" - x after {:?} {:?}", new_x, point.1);
                fold_point_lit.push(Point(new_x, point.1));
            }
        }
    }

    debug!("fold_point_lit {:?}", fold_point_lit);
    for fold_point in fold_point_lit {
        point_matrix[fold_point.1][fold_point.0] = '#'
    }
}
