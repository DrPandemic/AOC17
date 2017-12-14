use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn parse() -> Result<i32> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let mut in_garbage = false;
  let mut ignoring_next_char = false;
  let mut depth = 0;
  let mut score = 0;

  content.chars().for_each(|c| {
    match (c, in_garbage) {
      _ if ignoring_next_char == true => ignoring_next_char = false,
      ('!', true) => ignoring_next_char = true,
      ('<', false) => in_garbage = true,
      ('>', true) => in_garbage = false,
      (_, true) => score += 1,
      ('{', false) => depth += 1,
      ('}', false) => if depth > 0 { depth -= 1; },
      _ => {},
    }
  });

  Ok(score)
}

fn main() {
  println!("{}", parse().unwrap());
}
