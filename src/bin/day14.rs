use std::time::Duration;
use aoc2022::*;
use aoc2022::visualize::{Channel, Color, visualize};

fn to_pos(pos: &str) -> Pos2 {
  let (x, y) = pos.split_once(",").unwrap();
  Pos2::new(x.trim().parse().unwrap(), y.trim().parse().unwrap())
}

const DIRS: [Dir2; 3] = [Dir2::new(0, 1), Dir2::new(-1, 1), Dir2::new(1, 1)];

const EMPTY_COLOR: Color = (0, 0, 0);
const WALL_COLOR: Color = (0xff, 0xff, 0xff);
const SAND_COLOR: Color = (0xc2, 0xb2, 0x80);

fn drop_sand(map: &mut CharMap, mut pos: Pos2, floor: isize, channel: &Channel) -> bool {
  let view_off = Pos2::new(500 - floor, 0);
  while pos.y < floor - 1 {
    channel.draw_pixel(pos - view_off, SAND_COLOR);
    channel.sleep(Duration::from_micros(10));
    if let Some(d) = DIRS.iter().find(|d| map[pos + *d] == b'.') {
      channel.draw_pixel(pos - view_off, EMPTY_COLOR);
      pos += *d;
    } else {
      map[pos] = b'o';
      return true;
    }
  }
  map[pos] = b'o';
  false
}

fn color_fn(ch: u8) -> Color {
  match ch {
    b'.' => EMPTY_COLOR,
    b'#' => WALL_COLOR,
    b'o' => SAND_COLOR,
    _ => unreachable!(),
  }
}

fn solve(path: &str, channel: &Channel) -> (usize, usize) {
  let input = input_data(14, path);
  let lines = input
    .lines()
    .map(|line| line.split("->").map(to_pos).collect::<Vec<_>>())
    .collect::<Vec<Vec<_>>>();

  let positions = lines.iter().flat_map(|line| line.iter());
  let floor_y = positions.clone().map(|p| p.y).max().unwrap() + 2;
  let mut map = CharMap::empty(BoundsBehavior::grow(b'.'));

  lines.iter()
    .flat_map(|line| line.as_slice().windows(2))
    .flat_map(|line| line[0].line_to(line[1]))
    .for_each(|p| map[p] = b'#');
  map[Pos2::new(500, 0)] = b'.';

  channel.draw_map(&map, Pos2::new(500 - floor_y, 0), Pos2::new(500 + floor_y, floor_y), color_fn);

  let mut total = 0;
  while drop_sand(&mut map, Pos2::new(500, 0), floor_y, channel) {
    total += 1;
  }
  let first = total;

  // Count the last dropped
  total += 1;
  while map[Pos2::new(500, 0)] != b'o' {
    drop_sand(&mut map, Pos2::new(500, 0), floor_y, channel);
    total += 1;
  }
  (first, total)
}

#[test]
fn test() {
  assert_eq!((24, 93), solve("test.txt", &Channel::empty()));
  assert_eq!((618, 26358), solve("input.txt", &Channel::empty()));
}

fn main() {
  let test = solve("test.txt", &Channel::empty());
  println!("test.txt: {} and {}", test.0, test.1);

  visualize("input.txt", |channel| {
    let input = solve("input.txt", &channel);
    println!("test.txt: {} and {}", input.0, input.1);
  });
}
