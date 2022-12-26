use std::collections::HashMap;
use aoc2022::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

type WrapMap = HashMap<(Pos2, Dir), (Pos2, Dir)>;

#[derive(Clone, Copy, IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Hash, Debug)]
#[repr(usize)]
enum Dir {
  Right = 0,
  Down = 1,
  Left = 2,
  Up = 3,
}

impl Dir {
  fn right(self) -> Dir {
    (((self as usize) + 1) % 4).try_into().unwrap()
  }

  fn left(self) -> Dir {
    self.right().right().right()
  }

  fn reverse(self) -> Dir {
    self.right().right()
  }
}

const DIRS: [Dir2; 4] = [
  Dir2::new(1, 0),
  Dir2::new(0, 1),
  Dir2::new(-1, 0),
  Dir2::new(0, -1),
];

fn coord(idx: isize, size: isize, dir: Dir) -> Pos2 {
  match dir {
    Dir::Right => Pos2::new(size - 1, idx),
    Dir::Down => Pos2::new(size - 1 - idx, size - 1),
    Dir::Left => Pos2::new(0, size - 1 - idx),
    Dir::Up => Pos2::new(idx, 0),
  }
}

fn gen_match(map: &mut WrapMap, size: isize, pos1: Pos2, dir1: Dir, pos2: Pos2, dir2: Dir) {
  for idx in 0..size {
    map.insert((pos1 + coord(idx, size, dir1), dir1), (pos2 + coord(size - 1 - idx, size, dir2), dir2.reverse()));
    map.insert((pos2 + coord(idx, size, dir2), dir2), (pos1 + coord(size - 1 - idx, size, dir1), dir1.reverse()));
  }
}

fn flat_wraps(map: &CharMap) -> WrapMap {
  let mut wraps = HashMap::new();
  for y in 0..=map.bottom_right().y {
    let left = (0..).find(|x| map[Pos2::new(*x, y)] != b' ').unwrap();
    let right = (left..).find(|x| map[Pos2::new(*x, y)] == b' ').unwrap() - 1;
    wraps.insert((Pos2::new(left, y), Dir::Left), (Pos2::new(right, y), Dir::Left));
    wraps.insert((Pos2::new(right, y), Dir::Right), (Pos2::new(left, y), Dir::Right));
  }
  for x in 0..=map.bottom_right().x {
    let top = (0..).find(|y| map[Pos2::new(x, *y)] != b' ').unwrap();
    let bottom = (top..).find(|y| map[Pos2::new(x, *y)] == b' ').unwrap() - 1;
    wraps.insert((Pos2::new(x, top), Dir::Up), (Pos2::new(x, bottom), Dir::Up));
    wraps.insert((Pos2::new(x, bottom), Dir::Down), (Pos2::new(x, top), Dir::Down));
  }
  wraps
}

fn cube_wraps_test() -> WrapMap {
  const SIZE: isize = 4;
  const COORDS: [Pos2; 6] = [
    Pos2::new(2 * SIZE, 0),
    Pos2::new(0, SIZE),
    Pos2::new(SIZE, SIZE),
    Pos2::new(2 * SIZE, SIZE),
    Pos2::new(2 * SIZE, 2 * SIZE),
    Pos2::new(3 * SIZE, 2 * SIZE),
  ];
  let mut wraps = HashMap::new();
  gen_match(&mut wraps, SIZE, COORDS[0], Dir::Left, COORDS[2], Dir::Up);
  gen_match(&mut wraps, SIZE, COORDS[0], Dir::Up, COORDS[1], Dir::Up);
  gen_match(&mut wraps, SIZE, COORDS[0], Dir::Right, COORDS[5], Dir::Right);
  gen_match(&mut wraps, SIZE, COORDS[3], Dir::Right, COORDS[5], Dir::Up);
  gen_match(&mut wraps, SIZE, COORDS[5], Dir::Down, COORDS[1], Dir::Left);
  gen_match(&mut wraps, SIZE, COORDS[4], Dir::Down, COORDS[1], Dir::Down);
  gen_match(&mut wraps, SIZE, COORDS[4], Dir::Left, COORDS[2], Dir::Down);
  wraps
}

fn cube_wraps_input() -> WrapMap {
  const SIZE: isize = 50;
  const COORDS: [Pos2; 6] = [
    Pos2::new(SIZE, 0),
    Pos2::new(2 * SIZE, 0),
    Pos2::new(SIZE, SIZE),
    Pos2::new(0, 2 * SIZE),
    Pos2::new(SIZE, 2 * SIZE),
    Pos2::new(0, 3 * SIZE),
  ];
  let mut wraps = HashMap::new();
  gen_match(&mut wraps, SIZE, COORDS[0], Dir::Up, COORDS[5], Dir::Left);
  gen_match(&mut wraps, SIZE, COORDS[0], Dir::Left, COORDS[3], Dir::Left);
  gen_match(&mut wraps, SIZE, COORDS[1], Dir::Up, COORDS[5], Dir::Down);
  gen_match(&mut wraps, SIZE, COORDS[1], Dir::Right, COORDS[4], Dir::Right);
  gen_match(&mut wraps, SIZE, COORDS[1], Dir::Down, COORDS[2], Dir::Right);
  gen_match(&mut wraps, SIZE, COORDS[4], Dir::Down, COORDS[5], Dir::Right);
  gen_match(&mut wraps, SIZE, COORDS[3], Dir::Up, COORDS[2], Dir::Left);
  wraps
}

fn walk(map: &CharMap, mut pos: Pos2, mut dir: Dir, steps: usize, wraps: &WrapMap) -> (Pos2, Dir) {
  for _ in 0..steps {
    let (next_pos, next_dir) = wraps.get(&(pos, dir))
      .copied()
      .unwrap_or_else(|| (pos + DIRS[usize::from(dir)], dir));
    if map[next_pos] == b'#' {
      break;
    }
    pos = next_pos;
    dir = next_dir;
  }
  (pos, dir)
}


fn walk_map(map: &CharMap, mut cmds: &str, wraps: &WrapMap) -> (Pos2, Dir) {
  let x = (0..).find(|x| map[Pos2::new(*x, 0)] == b'.').unwrap();
  let mut pos = Pos2::new(x, 0);
  let mut dir = Dir::Right;
  while cmds != "" {
    if cmds.starts_with('R') {
      dir = dir.right();
      cmds = &cmds[1..];
    } else if cmds.starts_with('L') {
      dir = dir.left();
      cmds = &cmds[1..];
    } else {
      let idx = cmds.chars().position(|ch| !ch.is_digit(10)).unwrap_or(cmds.len());
      let steps = cmds[0..idx].parse::<usize>().unwrap();
      cmds = &cmds[idx..];
      (pos, dir) = walk(&map, pos, dir, steps, wraps);
    }
  }
  (pos, dir)
}

fn solve(path: &str) -> (isize, isize) {
  let input = input_data(22, path);
  let (map, cmds) = input.split_once("\n\n").unwrap();
  let cmds = cmds.trim();

  let map = CharMap::from_text(map).with_bounds(BoundsBehavior::abyss(b' '));
  let wraps = flat_wraps(&map);
  let (pos, dir) = walk_map(&map, cmds, &wraps);
  let first = 1000 * (pos.y + 1) + 4 * (pos.x + 1) + (dir as isize);

  let cube_wraps = if path == "test.txt" { cube_wraps_test() } else { cube_wraps_input() };
  let (pos, dir) = walk_map(&map, cmds, &cube_wraps);
  let second = 1000 * (pos.y + 1) + 4 * (pos.x + 1) + (dir as isize);
  (first, second)
}

#[test]
fn test() {
  assert_eq!((6032, 5031), solve("test.txt"));
  assert_eq!((191010, 55364), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
