use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn read_file() -> Result<Vec<Vec<String>>> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let lines = content.split('\n').map(|s| s.to_string()).filter(|s| s != "");
  Ok(lines.map(|line| line.split(' ').map(str::to_string).collect()).collect())
}

fn filter_passwords(input: Vec<Vec<String>>) -> Vec<Vec<String>> {
  input.into_iter().filter(|line| {
    let mut cleaned = line.clone();
    cleaned.sort();
    cleaned.dedup();

    println!("{:?} {:?}\n", line, cleaned);

    cleaned.len() == line.len()
  }).collect()
}

fn main() {
  println!("{:?}", filter_passwords(read_file().unwrap()).len());
}
