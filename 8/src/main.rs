use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

type Store = HashMap<String, i32>;

fn parse() -> Result<i32> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let mut data: Store = HashMap::new();
  let mut highest = 0;

  let lines = content.split('\n').map(|s| s.to_string()).filter(|s| s != "");
  lines.for_each(|l| {
    let elems: Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
    let dst = elems[0].clone();
    let action = elems[1].clone();
    let value = elems[2].parse::<i32>().unwrap();
    let cmp_register = elems[4].clone();
    let cmp_sign = elems[5].clone();
    let cmp_value = elems[6].parse::<i32>().unwrap();

    let cmp_content = get(&data, &cmp_register);

    match cmp_sign.as_ref() {
      ">" if cmp_content > cmp_value => perform(&mut data, dst, action, value),
      "<" if cmp_content < cmp_value => perform(&mut data, dst, action, value),
      ">=" if cmp_content >= cmp_value => perform(&mut data, dst, action, value),
      "<=" if cmp_content <= cmp_value => perform(&mut data, dst, action, value),
      "==" if cmp_content == cmp_value => perform(&mut data, dst, action, value),
      "!=" if cmp_content != cmp_value => perform(&mut data, dst, action, value),
      _ => {}
    };
    let current_highest =  *data.values().max().unwrap();
    highest = if current_highest > highest {
      current_highest
    } else {
      highest
    }
  });

  Ok(highest)
}

fn perform<'a>(data: &'a mut Store, key: String, action: String, value: i32) {
  let dst_content = get(&data, &key);
  let new_key = key.clone();
  match action.as_ref() {
    "inc" => data.insert(new_key, dst_content + value),
    "dec" => data.insert(new_key, dst_content - value),
    _ => panic!()
  };
}

fn get(data: &Store, key: &String) -> i32 {
  if data.contains_key(key) {
    data.get(key).unwrap().clone()
  } else {
    0
  }
}

fn main() {
    println!("{}", parse().unwrap());
}
