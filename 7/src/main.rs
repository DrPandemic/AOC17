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

fn total_weight(data: &Container, node: &Node) -> i32 {
  let weight: i32 = node.children.iter().map(|c| total_weight(data, get_node(data, c))).sum();
  node.weight + weight
}

fn find_unbalanced(data: &Container) {
  for node in data {
    let children: Vec<(&Node, i32)> = node.children.iter().map(|c| {
      let curr = get_node(data, c);
      (curr, total_weight(data, curr))
    }).collect();
    let min = children.iter().map(|e| e.1).min();
    let max = children.iter().map(|e| e.1).max();
    if min != max {
      let mut list: Vec<Vec<(&Node, i32)>> = vec![];

      for (child, weight) in children {
        // I don't like this
        let mut should_push = false;
        {
          let sub_list = list.iter_mut().find(|e| e.iter().any(|&(_, other_weight)| weight == other_weight));
          should_push = match sub_list {
            None => true,
            Some(the_list) => {
              the_list.push((child, weight));
              false
            },
          };
        }

        if should_push {
          list.push(vec![(child, weight)]);
        }
      }

      println!("{:?}", list);
    }
  }
}

fn main() {
  let data = read_file().unwrap();
  find_unbalanced(&data);
}
