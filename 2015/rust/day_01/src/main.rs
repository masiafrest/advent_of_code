use std::fs;

fn main() {
    part_one();
    part_two();
}

static FILEPATH: &str = "../../input_example/01_not_quite_lisp.txt";

pub fn get_text(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

fn part_one() {
    let data = get_text(FILEPATH);

    let mut sum = 0;

    // let v: Vec<&str> = data.split("").collect();
    // for c in v {
    //   match c {
    //     "(" => sum += 1,
    //     ")" => sum -= 1,
    //     _ => sum += 0
    //   }
    // }
    for c in data.chars() {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => sum += 0,
        }
    }

    println!("{}", sum);
}

fn part_two() {
    let text = get_text(FILEPATH);
    let mut sum = 0;
    let mut pos = 0;
    for (i, c) in text.chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => sum += 0,
        }
        if sum == -1 {
            pos = i + 1;
            break;
        }
    }
    println!("pos: {}", pos)
}