use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_imports)]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part 1
    let mut numbers: Vec<i32> = Vec::new();
    let mut matrix_list: Vec<Vec<i32>> = Vec::new();
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
            if row_count != 0 {
                println!("- {:?}, line {:?}", row_count, line);

                matrix_list.push(
                    line.unwrap()
                        .split(" ")
                        .filter(|x| !x.trim().is_empty())
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect(),
                );

                if row_count == 5 {
                    matrix_count += 1;
                }
            }
        }
    }

    println!("numbers {:?}", numbers);
    println!("matrix_list ({:?}) {:#?}", matrix_count, matrix_list);

    // let mut my_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..matrix_count {
        println!(
            "my_map  {:#?}",
            matrix_list[i as usize * 5..i as usize * 5 + 5]
        );
    }
    // println!("my_map  {:#?}", matrix_list[i as usize  * 5..i as usize  * 5 + 5]);
}
