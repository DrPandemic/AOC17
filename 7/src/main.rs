extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use regex::Regex;

type Container = Vec<Node>;

#[derive(Debug)]
struct Node {
  pub name: String,
  pub weight: i32,
  pub children: Vec<String>,
}

fn read_file() -> Result<Container> {
  let mut file = File::open("input")?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let lines = content.split('\n').map(|s| s.to_string()).filter(|s| s != "");
  let re = Regex::new(r"^(\w+) \((\d+)\)( -> (.*))?$").unwrap();

  Ok(lines.map(|l| {
    let groups = re.captures(&l).unwrap();

    Node{
      name: groups[1].to_owned(),
      weight: groups[2].to_owned().parse::<i32>().unwrap(),
      children: match groups.get(4) {
        Some(s) => s.as_str().split(", ").map(|v| v.to_owned()).collect(),
        _ => vec![]
      }
    }
  }).collect())
}

fn get_node<'a>(data: &'a Container, name: &String) -> &'a Node {
  data.iter().find(|n| &n.name == name).unwrap()
}

fn total_weight(data: &Container, name: &String) -> i32 {
  let node: &Node = get_node(data, name);
  let weight: i32 = node.children.iter().map(|c| total_weight(data, c)).sum();
  node.weight + weight
}

fn main() {
  let data = read_file().unwrap();
  println!("{:?}", data.iter().max_by(|a, b| total_weight(&data, &a.name).cmp(&total_weight(&data, &b.name))));
  println!("Hello, world!");
}
