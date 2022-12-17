use aoc2022::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Pattern(&'static [u8]);

const PATTERNS: [Pattern; 5] = [
  Pattern(&[0b11110000]),
  Pattern(&[0b01000000, 0b11100000, 0b01000000]),
  Pattern(&[0b11100000, 0b00100000, 0b00100000]),
  Pattern(&[0b10000000, 0b10000000, 0b10000000, 0b10000000]),
  Pattern(&[0b11000000, 0b11000000]),
];

fn has_overlap(state: &[u8], pat: usize, pos: Pos2) -> bool {
  let line = pos.y as usize;
  (0..PATTERNS[pat].0.len()).any(|idx| ((PATTERNS[pat].0[idx] >> pos.x) & state.get(line + idx).unwrap_or(&0)) != 0)
}

fn cache_key(state: &[u8], jet_idx: usize, pat_idx: usize) -> Option<Key> {
  let mut heights = [0usize; 7];
  for idx in 0..7 {
    let mask = 1 << (7 - idx);
    heights[idx] = state.iter().rev().take_while(|line| (*line & mask) == 0).count();
    if heights[idx] == state.len() {
      return None;
    }
  }
  Some(Key {
    heights,
    jet_idx,
    pat_idx,
  })
}

const DOWN: Dir2 = Dir2::new(0, -1);

fn step(state: &mut Vec<u8>, dir: u8, pat: usize, mut pos: Pos2) -> Pos2 {
  let x = pos.x as usize;
  let (mask, offset) = match dir {
    b'<' => (0b10000000, Dir2::new(-1, 0)),
    b'>' => (0b00000010, Dir2::new(1, 0)),
    _ => unreachable!(),
  };

  if PATTERNS[pat].0.iter().all(|p| ((p >> x) & mask) == 0) && !has_overlap(state, pat, pos + offset) {
    pos += offset;
  }
  if pos.y > 0 && !has_overlap(state, pat, pos + DOWN) {
    pos += DOWN;
  }
  pos
}

fn apply(state: &mut Vec<u8>, pat: usize, pos: Pos2) {
  let line = pos.y as usize;
  let new_len = (line + PATTERNS[pat].0.len()).max(state.len());
  state.resize(new_len, 0u8);
  for idx in 0..PATTERNS[pat].0.len() {
    assert_eq!(state[line + idx] & (PATTERNS[pat].0[idx] >> pos.x), 0);
    state[line + idx] |= PATTERNS[pat].0[idx] >> pos.x;
  }
}

fn highest(state: &[u8]) -> isize {
  state
    .iter()
    .rposition(|line| *line != 0)
    .map_or(-1, |line| (line as isize))
}

#[derive(PartialEq, Eq, Hash)]
struct Key {
  heights: [usize; 7],
  jet_idx: usize,
  pat_idx: usize,
}

enum Entry {
  Base(isize, usize),
  Diff(isize, usize),
}

fn solve_for(jets: &[u8], target: usize) -> isize {
  let mut cache: HashMap<Key, Entry> = HashMap::new();
  let mut state: Vec<u8> = Vec::new();
  let mut jet_it = (0..jets.len()).cycle().peekable();
  let mut idx = 0;
  let mut offset = 0isize;
  while idx < target {
    let pat = idx % 5;
    let height = highest(&state);
    let mut pos = Pos2::new(2, height + 4);

    // Loop detection
    if let Some(key) = cache_key(&state, *jet_it.peek().unwrap(), pat) {
      match cache.get(&key) {
        Some(Entry::Base(base, base_idx)) => {
          cache.insert(key, Entry::Diff(height - base, idx - base_idx));
        }
        Some(Entry::Diff(diff, diff_idx)) if target - idx >= *diff_idx => {
          let delta = (target - idx) / diff_idx;
          offset += (delta as isize) * (*diff);
          idx += delta * diff_idx;
        }
        Some(_) => {}
        None => {
          cache.insert(key, Entry::Base(height, idx));
        }
      }
    }

    loop {
      let jet_idx = jet_it.next().unwrap();
      let old_y = pos.y;
      pos = step(&mut state, jets[jet_idx], pat, pos);
      if pos.y == old_y {
        apply(&mut state, pat, pos);
        break;
      }
    }
    idx += 1;
  }
  offset + highest(&state) + 1
}

fn solve(path: &str) -> (isize, isize) {
  let jets = input_data(17, path);
  let jets = jets.trim().as_bytes();

  let first = solve_for(jets, 2022);
  let second = solve_for(jets, 1000000000000);
  (first, second)
}

#[test]
fn test() {
  assert_eq!((3068, 1514285714288), solve("test.txt"));
  assert_eq!((3227, 1597714285698), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
