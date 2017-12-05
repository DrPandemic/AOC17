use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

type Container = Vec<i32>;

fn read_file() -> Result<Container> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let lines = content.split('\n').map(|s| s.to_string()).filter(|s| s != "");
  Ok(lines.map(|l| l.parse::<i32>().unwrap()).collect())
}

fn run(input: Container) -> i32 {
  let mut i = 0;
  let mut ptr = 0;
  let mut mut_input = input.clone();
  let mut old = 0;
  loop {
    i += 1;
    old = ptr;
    ptr += mut_input[ptr as usize];
    if ptr >= input.len() as i32 {
      break
    }
    mut_input[old as usize] += if mut_input[old as usize] >= 3 { -1 } else { 1 };
  }

  i
}

fn main() {
  println!("{:?}", run(read_file().unwrap()));
}
