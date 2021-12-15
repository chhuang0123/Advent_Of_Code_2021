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
}

fn get_lowest_total_risk(m: usize, n: usize, risk: &Vec<Vec<usize>>) -> usize {
    let row_count = risk.len();
    let column_count = risk[0].len();
    let mut total_risk: Vec<Vec<usize>> = vec![vec![0_usize; column_count]; row_count];

    for i in 1..row_count {
        total_risk[0][i] = total_risk[0][i - 1] + risk[0][i] as usize;
    }

    for j in 1..row_count {
        total_risk[j][0] = total_risk[j - 1][0] + risk[j][0] as usize;
    }

    for i in 1..row_count {
        for j in 1..column_count {
            total_risk[j][i] = vec![total_risk[j - 1][i], total_risk[j][i - 1]]
                .iter()
                .min()
                .unwrap()
                + risk[j][i] as usize;
        }
    }

    for i in 0..row_count {
        for j in 0..column_count {
            print!("{:02} ", total_risk[i][j]);
        }
        println!();
    }

    total_risk[n - 1][m - 1]
}
