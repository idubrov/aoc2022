use std::cmp::Ordering;
use aoc2022::*;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
  y: isize,
  x: isize,
  cost: usize,
}

impl Ord for Pos {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
      .then_with(|| self.x.cmp(&other.x))
      .then_with(|| self.y.cmp(&other.y))
  }
}

impl PartialOrd for Pos {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn solve(path: &str) {
  println!("{} (first): {}", path, find_path(path, false));
  println!("{} (second): {}", path, find_path(path, true));
}

fn find_path(path: &str, any: bool) -> usize {
  let input = input_data(12, path);
  let map = input.lines()
    .map(|s| s.as_bytes().to_vec())
    .collect::<Vec<_>>();

  let srow = map.iter().position(|row| row.contains(&b'E')).unwrap() as isize;
  let scol = map[srow as usize].iter().position(|c| c == &b'E').unwrap() as isize;

  let mut costs = vec![vec![map.len() * map[0].len(); map[0].len()]; map.len()];
  let mut queue = BinaryHeap::new();
  queue.push(Pos {
    y: srow,
    x: scol,
    cost: 0,
  });
  while let Some(Pos { cost, x, y }) = queue.pop() {
    if cost >= costs[y as usize][x as usize] {
      continue;
    }
    costs[y as usize][x as usize] = cost;
    let mut src = map[y as usize][x as usize];
    if src == b'E' {
      src = b'z';
    }
    if src == b'S' || (src == b'a' && any) {
      return cost;
    }
    for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
      let tx = x + dx;
      let ty = y + dy;
      if tx >= 0 && tx < map[0].len() as isize && ty >= 0 && ty < map.len() as isize {
        let mut tgt = map[ty as usize][tx as usize];
        if tgt == b'S' {
          tgt = b'a';
        }
        if src - 1 <= tgt {
          queue.push(Pos {
            x: tx,
            y: ty,
            cost: cost + 1,
          });
        }
      }
    }
  }
  unreachable!();
}


fn main() {
  solve("test.txt");
  solve("input.txt");
}
