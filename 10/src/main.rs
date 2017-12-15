use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn parse() -> Result<i32> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let mut position = 0;
  let mut skip = 0;
  let mut lengths: Vec<i32> = content.split(',').map(|s| s.to_string()).filter(|s| s != "").map(|n| n.parse::<i32>().unwrap()).collect();
  let mut list: Vec<i32> = (0..256).collect();

  Ok(list[0] * list[1])
}

fn reverse(data: &mut Vec<i32>, start_position: i32, length: i32) {
  let mut start = start_position as usize;
  let mut end = (start as i32 + length - 1) % data.len() as i32;
  let mut i = length / 2;

  while i > 0 {
    i -= 1;
    let tmp = data[start];
    data[start] = data[end as usize];
    data[end as usize] = tmp;

    start += 1;
    start %= data.len() as usize;
    end -= 1;
    end = if end < 0 { data.len() as i32 - 1 } else { end };
  }
}

fn main() {
  let mut data = vec![0, 1, 2, 3, 4, 5, 6];
  reverse(&mut data, 4, 5);
  println!("{:?}", data);
  // println!("{}", parse().unwrap());
}
