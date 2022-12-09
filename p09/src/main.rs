use std::collections::HashSet;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
  x: i32,
  y: i32,
}

type Rope = Vec<Pos>;

fn advance(rope: &mut Rope) {
  for idx in 0..rope.len() - 1 {
    let dx = rope[idx].x - rope[idx + 1].x;
    let dy = rope[idx].y - rope[idx + 1].y;
    if dx >= 2 || dx <= -2 || dy >= 2 || dy <= -2 {
      rope[idx + 1].x += dx.signum();
      rope[idx + 1].y += dy.signum();
    }
  }
}
fn solve(input: &str, mut rope: Vec<Pos>) -> usize {
  let mut tails = HashSet::new();
  for line in input.lines() {
    let mut it = line.splitn(2, " ");
    let cmd = it.next().unwrap();
    let dist = it.next().unwrap();
    let (dx, dy) = match cmd {
      "R" => (1, 0),
      "U" => (0, 1),
      "L" => (-1, 0),
      "D" => (0, -1),
      _ => unreachable!(),
    };

    for _ in 0..dist.parse::<usize>().unwrap() {
      rope[0].x += dx;
      rope[0].y += dy;
      advance(&mut rope);
      tails.insert(*rope.last().unwrap());
    }
  }
  tails.len()
}

fn main() {
  let input = std::fs::read_to_string("p09/src/input.txt").unwrap();
  eprintln!("{}", solve(&input, vec![Pos::default(); 2]));
  eprintln!("{}", solve(&input, vec![Pos::default(); 10]));
}
