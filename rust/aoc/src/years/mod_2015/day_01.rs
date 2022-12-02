use aoc::get_input_example;

pub fn run() {
    let data = get_input_example(2015, 1);
    part_one(&data);
    part_two(&data);
}
fn part_one(data: &str) {
    let mut sum = 0;

    for c in data.chars() {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => sum += 0,
        }
    }

    println!("sum: {}", sum);
}

fn part_two(data: &str) {
    let mut sum = 0;
    let mut pos = 0;
    for (i, c) in data.chars().enumerate() {
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
