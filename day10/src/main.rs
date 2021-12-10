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
    for chunk in chunks {
        if let Err(err) = check_chunk(&chunk) {
            debug!("err {:?}", err);
            let point = get_illegal_point(err);
            debug!("point {:?}", point);
            illegal_point += point;
        };
    }
    info!("part 1: {:?}", illegal_point);
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

fn check_chunk(chunk: &Vec<char>) -> Result<usize, char> {
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
    Ok(check_stack.len())
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

fn check_match(left_char: char, right_char: char) -> bool {
    if left_char == '(' && right_char == ')' {
        return true;
    }

    if left_char == '[' && right_char == ']' {
        return true;
    }

    if left_char == '{' && right_char == '}' {
        return true;
    }

    if left_char == '<' && right_char == '>' {
        return true;
    }

    false
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
    let expect = Ok(0);
    let actual = check_chunk(&"([])".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(0);
    let actual = check_chunk(&"{()()()}".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(0);
    let actual = check_chunk(&"<([{}])>".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(0);
    let actual = check_chunk(&"[<>({}){}[([])<>]]".chars().collect());
    assert_eq!(expect, actual);

    let expect = Ok(0);
    let actual = check_chunk(&"(((((((((())))))))))".chars().collect());
    assert_eq!(expect, actual);
}

#[test]
fn test_check_chunk_ok_len() {
    let expect = Ok(8);
    let actual = check_chunk(&"[({(<(())[]>[[{[]{<()<>>".chars().collect());
    assert_eq!(expect, actual);
}
