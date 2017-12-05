use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn read_file() -> Result<Vec<Vec<Vec<char>>>> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let lines = content.split('\n').map(|s| s.to_string()).filter(|s| s != "");
  Ok(lines.map(
    |line| line.split(' ').map(
      |word| word.chars().collect()
    ).collect()
  ).collect())
}

fn filter_passwords(input: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
  input.into_iter().filter(|line| {
    let mut cleaned = line.clone();
    for ref mut word in &mut cleaned {
      word.sort();
    }
    cleaned.sort();
    cleaned.dedup();

    cleaned.len() == line.len()
  }).collect()
}

fn main() {
  println!("{:?}", filter_passwords(read_file().unwrap()).len());
}
