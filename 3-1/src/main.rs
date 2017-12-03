const INPUT: i32 = 361527;
// const INPUT: i32 = 44;

#[derive(Clone, Debug)]
enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn distance(movements: Vec<Movement>) -> i32 {
  let (up, down, left, right) = movements.into_iter().fold(
    (0i32, 0i32, 0i32, 0i32),
    |(up, down, left, right), m| match m {
      Movement::Up => (up + 1, down, left, right),
      Movement::Down => (up, down + 1, left, right),
      Movement::Left => (up, down, left + 1, right),
      Movement::Right => (up, down, left, right + 1),
    }
  );

  (up - down).abs() + (left - right).abs()
}

fn travel(end: i32) -> Vec<Movement> {
  let mut i = 1;
  let mut movements: Vec<Movement> = vec![];
  let mut last = Movement::Down;
  let mut serie = 0;
  let mut in_serie = 1;
  let mut skip = false;

  while i < end {
    let movement = if in_serie >= serie {
      in_serie = 1;
      serie += if skip {0} else {1};
      skip = !skip;
      match last {
        Movement::Right => Movement::Up,
        Movement::Up => Movement::Left,
        Movement::Left => Movement::Down,
        Movement::Down => Movement::Right,
      }
    } else {
      in_serie += 1;
      last.clone()
    };
    movements.push(movement.clone());
    last = movement;
    i += 1;
  }

  movements
}

fn main() {
  let result = distance(travel(INPUT));
  println!("{:?}", result);
}
