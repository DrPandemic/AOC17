use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
extern crate itertools;
use itertools::Itertools;

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
  let (a, b) = line.iter().cartesian_product(line.iter()).find(|&(a, b)| a != b && a % b == 0).unwrap();
  a / b
}

fn main() {
  let content = get_content().unwrap();
  println!("{:?}", content.iter().map(|line| checksum(line)).sum::<i32>());
}
