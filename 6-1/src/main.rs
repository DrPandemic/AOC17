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

fn run(input: Container) -> i32 {
  let mut i = 0;
  let mut input_mut = input.clone();
  let mut seen: Vec<Container> = vec![];

  'outer: loop {
    seen.push(input_mut.clone());
    i += 1;
    let (mut position, mut val) = (&input_mut).into_iter().enumerate().fold((0 as usize, 0 as i32), |(i, val), (i_curr, &val_curr)| {
      if val_curr > val {
        (i_curr, val_curr)
      } else {
        (i, val)
      }
    });
    input_mut[position] = 0;
    while val > 0 {
      position = (position + 1) % input_mut.len();
      input_mut[position] += 1;
      val -= 1;

      match (&seen).into_iter().find(|&elem| elem == &input_mut) {
        Some(_) => break 'outer,
        _ => {}
      }
    }
  }

  i
}

fn main() {
  println!("{:?}", run(read_file().unwrap()));
}
