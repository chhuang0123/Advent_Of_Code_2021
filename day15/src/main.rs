use log::{debug, info};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("filename {}", filename);

    let mut risk: Vec<Vec<usize>> = Vec::new();
    for line in contents.lines() {
        debug!("line {}", line);
        risk.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        );
    }
    debug!("{:?}", risk);

    let row_count = risk.len();
    let column_count = risk[0].len();
    debug!("row_count {:?} column_count {:?}", row_count, column_count);

    let result = get_lowest_total_risk(row_count, column_count, &risk);
    info!("part 1: {:?}", result);

    let mut five_risk: Vec<Vec<usize>> = vec![vec![0; column_count * 5]; row_count * 5];
    for i in 0..row_count * 5 {
        for j in 0..column_count * 5 {
            let mut result =
                risk[i % row_count][j % column_count] + (i / row_count) + (j / row_count);
            if result > 9 {
                result %= 9;
            }
            five_risk[i][j] = result;
        }
    }
    debug!("{:?}", five_risk);

    let row_count = five_risk.len();
    let column_count = five_risk[0].len();
    debug!("row_count {:?} column_count {:?}", row_count, column_count);

    let result = get_lowest_total_risk(row_count, column_count, &five_risk);
    info!("part 2: {:?}", result);
}

fn get_lowest_total_risk(m: usize, n: usize, risk: &Vec<Vec<usize>>) -> usize {
    let row_count = risk.len();
    let column_count = risk[0].len();
    let mut total_risk: Vec<Vec<usize>> = vec![vec![0_usize; column_count]; row_count];

    for i in 1..row_count {
        total_risk[0][i] = total_risk[0][i - 1] + risk[0][i];
    }

    for j in 1..row_count {
        total_risk[j][0] = total_risk[j - 1][0] + risk[j][0];
    }

    for i in 1..row_count {
        for j in 1..column_count {
            total_risk[j][i] = vec![total_risk[j - 1][i], total_risk[j][i - 1]]
                .iter()
                .min()
                .unwrap()
                + risk[j][i];
        }
    }

    total_risk[n - 1][m - 1]
}
