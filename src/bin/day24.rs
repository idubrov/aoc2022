use aoc2022::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Default, Debug)]
struct Winds {
  lefts: Vec<u128>,
  rights: Vec<u128>,
  ups: Vec<u128>,
  downs: Vec<u128>,
}

impl Winds {
  fn allowed(&self, dest: Pos2, step: usize) -> bool {
    let w = self.ups.len() as isize;
    let h = self.lefts.len() as isize;
    let left = rol(self.lefts[dest.y as usize], step, w as usize) & (1 << dest.x);
    let right = ror(self.rights[dest.y as usize], step, w as usize) & (1 << dest.x);
    let up = rol(self.ups[dest.x as usize], step, h as usize) & (1 << dest.y);
    let down = ror(self.downs[dest.x as usize], step, h as usize) & (1 << dest.y);
    (left | right | up | down) == 0
  }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct PathState {
  pos: Pos2,
  step: usize,
}

impl Ord for PathState {
  fn cmp(&self, other: &Self) -> Ordering {
    other.step.cmp(&self.step).then_with(|| self.pos.cmp(&other.pos))
  }
}

impl PartialOrd for PathState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn scan(map: &CharMap, start: Pos2, dir: Dir2, ch: u8, steps: isize) -> u128 {
  let mut pos = start;
  let mut bits = 0;
  for idx in 0..steps {
    if map[pos] == ch {
      bits |= 1 << idx;
    }
    pos += dir;
  }
  bits
}

fn rol(value: u128, mut shift: usize, width: usize) -> u128 {
  shift %= width;
  let mask = (1 << shift) - 1;
  (value >> shift) | ((value & mask) << (width - shift))
}

fn ror(value: u128, mut shift: usize, width: usize) -> u128 {
  shift %= width;
  rol(value, width - shift, width)
}

fn bfs(winds: &Winds, start: Pos2, step: usize, end: Pos2) -> usize {
  let low = Pos2::new(0, 0);
  let high = Pos2::new(winds.ups.len() as isize - 1, winds.lefts.len() as isize - 1);
  let mut visited = HashSet::new();
  let mut queue = BinaryHeap::new();
  queue.push(PathState { pos: start, step });
  while let Some(PathState { pos, mut step }) = queue.pop() {
    if !visited.insert(PathState { pos, step }) {
      continue;
    }
    if pos == end {
      return step;
    }

    step += 1;

    if !pos.inside_rect(low, high) || winds.allowed(pos, step) {
      queue.push(PathState { pos, step });
    }

    for dir in Dir2::all_4() {
      let dest = pos + dir;
      if dest.inside_rect(low, high) && winds.allowed(dest, step) {
        queue.push(PathState { pos: dest, step });
      }
    }
  }
  unreachable!();
}

fn solve(path: &str) -> (usize, usize) {
  let input = input_data(24, path);
  let map = CharMap::from_text(&input);
  let w = map.bottom_right().x - 1;
  let h = map.bottom_right().y - 1;
  let mut winds = Winds::default();
  for y in 0..h {
    winds
      .lefts
      .push(scan(&map, Pos2::new(1, 1 + y), Dir2::new(1, 0), b'<', w));
    winds
      .rights
      .push(scan(&map, Pos2::new(1, 1 + y), Dir2::new(1, 0), b'>', w));
  }
  for x in 0..w {
    winds
      .ups
      .push(scan(&map, Pos2::new(1 + x, 1), Dir2::new(0, 1), b'^', h));
    winds
      .downs
      .push(scan(&map, Pos2::new(1 + x, 1), Dir2::new(0, 1), b'v', h));
  }
  let start = Pos2::new(0, -1);
  let end = Pos2::new(w - 1, h);
  let start_target = Pos2::new(0, 0);
  let end_target = Pos2::new(w - 1, h - 1);
  let first = bfs(&winds, start, 0, end_target) + 1;
  let back = bfs(&winds, end, first, start_target) + 1;
  let second = bfs(&winds, start, back, end_target) + 1;
  (first, second)
}

#[test]
fn test() {
  assert_eq!((18, 54), solve("test.txt"));
  assert_eq!((301, 859), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
