use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::cmp;

fn read_file() -> Result<String> {
  let mut file = File::open("input")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn parse(input: String) -> Vec<Vec<i32>> {
  let lines = input.split('\n').map(|s| s.to_string());
  lines.map(
    |line| line.split('\t').map(
      |column| match column.parse::<i32>() {
        Ok(number) => number,
        _ => 0,
      }).collect()
  ).collect()
}

fn get_content() -> Result<Vec<Vec<i32>>> {
  let content = read_file()?;
  Ok(parse(content))
}

fn checksum(line: &Vec<i32>) -> i32 {
  let (min, max) = line.iter().fold(
    (<i32>::max_value(), <i32>::min_value()),
    |(min, max), &curr| (cmp::min(min, curr), cmp::max(max, curr))
  );
  max - min
}

fn main() {
  let content = get_content().unwrap();
  println!("{:?}", content.iter().map(|line| checksum(line)).sum::<i32>());
}
