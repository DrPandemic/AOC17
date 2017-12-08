use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

type Container = Vec<i32>;

fn read_file() -> Result<Container> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let lines: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
  Ok(lines[0].split('\t').map(
    |column| match column.parse::<i32>() {
      Ok(number) => number,
      _ => 0,
    }).collect())
}

fn main() {
    println!("Hello, world!");
}
