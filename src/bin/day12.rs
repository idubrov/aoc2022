use aoc2022::*;

fn cost_fn(map: &CharMap, from: Pos2, to: Pos2) -> Option<usize> {
  if map[from] <= map[to] + 1 {
    Some(1)
  } else {
    None
  }
}

fn solve(path: &str) {
  let input = input_data(12, path);
  let mut map = CharMap::from_text(&input);

  let start = map.every_pos().find(|pos| map[*pos] == b'S').unwrap();
  let end = map.every_pos().find(|pos| map[*pos] == b'E').unwrap();

  map[start] = b'a';
  map[end] = b'z';

  let from_start = map.find_path(end, |_, pos| pos == start, cost_fn).unwrap();
  let from_any_start = map.find_path(end, |_, pos| map[pos] == b'a', cost_fn).unwrap();

  println!("{} (first): {}", path, from_start);
  println!("{} (second): {}", path, from_any_start);
}

fn main() {
  solve("test.txt");
  solve("input.txt");
}
