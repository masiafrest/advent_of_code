use aoc::get_input_example;

pub fn run() {
  let data = get_input_example(2022, 1);
  let data = data.split("\n");
  let mut sums : Vec<i32> = Vec::new();
  let mut sum  = 0;
  for line in data.clone() {
    if line.is_empty() {
      sums.push(sum);
      sum = 0;
      continue
    }
    println!("{:?}", line);
    sum += line.parse::<i32>().unwrap();
  }
  println!("sum {:?}, total {:?}", sum, sums)
}