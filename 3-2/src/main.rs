const INPUT: i32 = 361527;
// const INPUT: i32 = 44;

#[derive(Clone, Debug)]
enum Movement {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Debug)]
struct Point {
  x: i32,
  y: i32,
  value: i32,
  movement: Movement,
}

impl Point {
  fn next(&self, movement: Movement, points: &Vec<Point>) -> Point {
    // println!("{:?}", points);
    let y = match self.movement {
      Movement::Up => self.y + 1,
      Movement::Down => self.y - 1,
      _ => self.y,
    };
    let x = match self.movement {
      Movement::Left => self.x - 1,
      Movement::Right => self.x + 1,
      _ => self.x,
    };
    Point {
      x: x,
      y: y,
      value: find_value_at_pos(x, y, points),
      movement: movement,
    }
  }
}

fn is_around(x0: i32, x1: i32, y0: i32, y1: i32) -> bool {
  (x0 - 1 == x1 && y0 - 1 == y1) ||
    (x0 == x1 && y0 - 1 == y1) ||
    (x0 + 1 == x1 && y0 - 1 == y1) ||
    (x0 - 1 == x1 && y0 == y1) ||
    (x0 + 1 == x1 && y0 == y1) ||
    (x0 - 1 == x1 && y0 + 1 == y1) ||
    (x0 == x1 && y0 + 1 == y1) ||
    (x0 + 1 == x1 && y0 + 1 == y1)
}

fn find_value_at_pos(x: i32, y: i32, points: &Vec<Point>) -> i32 {
  points.into_iter().filter(|&&Point {x: x1, y: y1, ..}| is_around(x, x1, y, y1)).fold(0, |acc, curr| {
    // println!("{:?}", curr);
    acc + curr.value
  })
}

fn travel(end: i32) -> Vec<Point> {
  let mut last = Point { x: 0, y: 0, value: 1, movement: Movement::Right };
  let mut points: Vec<Point> = vec![last.clone()];
  let mut serie = 1;
  let mut in_serie = 1;
  let mut skip = true;

  while last.value < end {
    let next_movement = if in_serie >= serie {
      in_serie = 1;
      serie += if skip {0} else {1};
      skip = !skip;
      match last.movement {
        Movement::Right => Movement::Up,
        Movement::Up => Movement::Left,
        Movement::Left => Movement::Down,
        Movement::Down => Movement::Right,
      }
    } else {
      in_serie += 1;
      last.movement.clone()
    };
    let next_point = last.next(next_movement, &points);

    // println!("{:?}", next_point);

    points.push(next_point.clone());
    last = next_point;
  }

  points
}

fn main() {
  let result = travel(INPUT + 1);
  println!("{:?}", result);
}
