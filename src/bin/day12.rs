use aoc2022::visualize::{visualize, Channel};
use aoc2022::*;
use std::time::Duration;

fn cost_fn(map: &CharMap, from: Pos2, to: Pos2) -> Option<usize> {
  if map[from] + 1 >= map[to] {
    Some(1)
  } else {
    None
  }
}

fn visualize_fn(visualize: &Channel) -> impl Fn(&CharMap, VisitKind, Pos2, usize) + '_ {
  |map, visit, pos, _cost| {
    let item = map[pos];
    let color = match visit {
      VisitKind::Consider => (0xff, 0, 0),
      VisitKind::Visit => (0, (255 * u32::from(item - b'a') / 26) as u8, 0),
    };
    visualize.draw_pixel(pos, color);
    visualize.sleep(Duration::from_nanos(1000));
  }
}

fn solve(path: &str, visualize: &Channel) {
  let input = input_data(12, path);
  let mut map = CharMap::from_text(&input);

  let start = map.every_pos().find(|pos| map[*pos] == b'S').unwrap();
  let end = map.every_pos().find(|pos| map[*pos] == b'E').unwrap();

  map[start] = b'a';
  map[end] = b'z';

  visualize.draw_map(&map, |item| (0, (255 * u32::from(item - b'a') / 26) as u8, 0));
  let from_start = map
    .find_path_cb(start, |_, pos| pos == end, cost_fn, visualize_fn(visualize))
    .unwrap();
  let from_any_start = map
    .find_path(end, |_, pos| map[pos] == b'a', |map, from, to| cost_fn(map, to, from))
    .unwrap();

  println!("{} (first): {}", path, from_start);
  println!("{} (second): {}", path, from_any_start);
}

fn main() {
  solve("test.txt", &Channel::empty());
  solve("input.txt", &Channel::empty());
  // visualize("day 12", |channel| {
  //   solve("input.txt", &channel);
  // })
}
