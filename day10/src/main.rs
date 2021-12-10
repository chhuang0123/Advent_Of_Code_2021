use log::{debug, info};
use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let filename = env::args().nth(1).expect("no filename given!");
    let contents = fs::read_to_string(&filename).unwrap();
    debug!("{}", filename);

    let mut chunks: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        debug!("{:?}", line);

        let chunk = line.chars().collect();
        debug!("{:?}", chunk);

        chunks.push(chunk);
    }
    debug!("{:?}", chunks);

    let mut illegal_point: u32 = 0;
    let mut incomplete_point: Vec<u64> = Vec::new();
    for chunk in chunks {
        let result = check_chunk(&chunk);
        match result {
            Err(err) => {
                debug!("err {:?}", err);
                let point = get_illegal_point(err);
                debug!("point {:?}", point);
                illegal_point += point;
            }
            Ok(ok) => {
                if ok.len() != 0 {
                    incomplete_point.push(get_sum_illegal_point(&ok));
                }
            }
        };
    }
    info!("part 1: {:?}", illegal_point);

    debug!("part 2 incomplete_point: {:?}", incomplete_point);
    incomplete_point.sort();
    debug!("sorted incomplete_point: {:?}", incomplete_point);

    let middle_index: usize = (incomplete_point.len() as f64 / 2_f64).floor() as usize;
    debug!("middle_index: {:?}", middle_index);
    info!("part 2: middle score: {:?}", incomplete_point[middle_index]);
}

fn get_complete_point(right_char: char) -> u32 {
    match right_char {
        ')' => return 1,
        ']' => return 2,
        '}' => return 3,
        '>' => return 4,
        _ => return 0,
    }
}

fn get_illegal_point(right_char: char) -> u32 {
    match right_char {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _ => return 0,
    }
}

fn get_sum_illegal_point(incomplete_chunk: &Vec<char>) -> u64 {
    debug!("incomplete_chunk {:?}", incomplete_chunk);

    let mut result: u64 = 0;
    for c in incomplete_chunk {
        result = result * 5_u64 + get_complete_point(*c) as u64;
    }

    debug!("final sum {:?}", result);
    result
}

fn check_chunk(chunk: &Vec<char>) -> Result<Vec<char>, char> {
    debug!("check_chunk {:?}", chunk);

    let mut check_stack: Vec<char> = Vec::new();
    for tmp_char in chunk {
        debug!("{:?}", tmp_char);
        if *tmp_char == '(' || *tmp_char == '[' || *tmp_char == '{' || *tmp_char == '<' {
            check_stack.push(*tmp_char);
        } else {
            let last_char = check_stack.pop();
            let match_char = get_match(last_char.unwrap());
            if match_char != *tmp_char {
                return Err(*tmp_char);
            }
        }
    }

    debug!("check_stack {:?}", check_stack);
    check_stack.reverse();
    let incomplete_chunk = get_incomplete_chunk(&check_stack);
    debug!("incomplete_chunk {:?}", incomplete_chunk);

    Ok(incomplete_chunk)
}

fn get_match(left_char: char) -> char {
    match left_char {
        '(' => return ')',
        '[' => return ']',
        '{' => return '}',
        '<' => return '>',
        _ => return ' ',
    }
}

fn get_incomplete_chunk(chunk: &Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for c in chunk {
        result.push(get_match(*c));
    }

    result
}

#[test]
fn test_check_chunk_err() {
    let expect = Err('}');
    let actual = check_chunk(&"{([(<{}[<>[]}>{[]{[(<()>".chars().collect());
    assert_eq!(expect, actual);

    let expect = Err(')');
    let actual = check_chunk(&"[[<[([]))<([[{}[[()]]]".chars().collect());
    assert_eq!(expect, actual);

    let expect = Err(']');
    let actual = check_chunk(&"[{[{({}]{}}([{[{{{}}([]".chars().collect());
    assert_eq!(expect, actual);

    let expect = Err(')');
    let actual = check_chunk(&"[<(<(<(<{}))><([]([]()".chars().collect());
    assert_eq!(expect, actual);

    let expect = Err('>');
    let actual = check_chunk(&"<{([([[(<>()){}]>(<<{{".chars().collect());
    assert_eq!(expect, actual);
}

#[test]
fn test_check_chunk_ok() {
    let expect = Ok(vec![]);
    let actual = check_chunk(&"([])".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(vec![]);
    let actual = check_chunk(&"{()()()}".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(vec![]);
    let actual = check_chunk(&"<([{}])>".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(vec![]);
    let actual = check_chunk(&"[<>({}){}[([])<>]]".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(vec![]);
    let actual = check_chunk(&"(((((((((())))))))))".chars().collect());
    assert_eq!(expect, actual);
}

#[test]
fn test_get_sum_illegal_point() {
    let expect = 294_u64;
    let actual = get_sum_illegal_point(&"])}>".chars().collect());
    assert_eq!(expect, actual);

    let expect = 288957_u64;
    let actual = get_sum_illegal_point(&"}}]])})]".chars().collect());
    assert_eq!(expect, actual);
}
