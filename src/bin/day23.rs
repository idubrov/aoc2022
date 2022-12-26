use aoc2022::*;
use std::collections::{BTreeSet, HashMap};

const DIRS: [[Dir2; 3]; 4] = [
  [Dir2::new(0, -1), Dir2::new(1, -1), Dir2::new(-1, -1)],
  [Dir2::new(0, 1), Dir2::new(1, 1), Dir2::new(-1, 1)],
  [Dir2::new(-1, 0), Dir2::new(-1, -1), Dir2::new(-1, 1)],
  [Dir2::new(1, 0), Dir2::new(1, -1), Dir2::new(1, 1)],
];

fn advance(elves: &BTreeSet<Pos2>, offset: usize) -> BTreeSet<Pos2> {
  let mut moves = HashMap::with_capacity(elves.len());
  for elf in elves {
    if Dir2::all_8().any(|dir| elves.contains(&(*elf + dir))) {
      // Need to move
      for idx in 0..4 {
        let dir = (idx + offset) % 4;
        if DIRS[dir].iter().all(|d| !elves.contains(&(*elf + *d))) {
          moves.insert(elf, DIRS[dir][0]);
          break;
        }
      }
    }
  }

  let mut dest: HashMap<Pos2, usize> = HashMap::new();
  for elf in elves {
    let target = *elf + moves.get(&elf).copied().unwrap_or_default();
    *dest.entry(target).or_default() += 1;
  }

  let mut next = BTreeSet::new();
  for elf in elves {
    let target = *elf + moves.get(&elf).copied().unwrap_or_default();
    if dest.get(&target) == Some(&1) {
      next.insert(target);
    } else {
      next.insert(*elf);
    }
  }

  next
}

fn solve(path: &str) -> (isize, isize) {
  let input = input_data(23, path);
  let map = CharMap::from_text(&input);

  let mut elves = map.every_pos().filter(|p| map[*p] == b'#').collect::<BTreeSet<_>>();

  let mut first = 0;
  let mut second = 0;
  for idx in 0.. {
    if idx == 10 {
      let min_x = elves.iter().map(|elf| elf.x).min().unwrap();
      let max_x = elves.iter().map(|elf| elf.x).max().unwrap();
      let min_y = elves.iter().map(|elf| elf.y).min().unwrap();
      let max_y = elves.iter().map(|elf| elf.y).max().unwrap();
      first = (max_x - min_x + 1) * (max_y - min_y + 1) - (elves.len() as isize);
    }
    let next = advance(&elves, idx);
    if next == elves {
      second = (idx + 1) as isize;
      break;
    }
    elves = next;
  }

  (first, second)
}

#[test]
fn test() {
  assert_eq!((0, 4), solve("small.txt"));
  assert_eq!((110, 20), solve("test.txt"));
  assert_eq!((3906, 895), solve("input.txt"));
}

fn main() {
  let test = solve("small.txt");
  println!("small.txt: {} and {}", test.0, test.1);

  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
