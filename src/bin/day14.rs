use aoc2022::*;

fn to_pos(pos: &str) -> Pos2 {
  let (x, y) = pos.split_once(",").unwrap();
  Pos2::new(x.trim().parse().unwrap(), y.trim().parse().unwrap())
}

const DIRS: [Dir2; 3] = [Dir2::new(0, 1), Dir2::new(-1, 1), Dir2::new(1, 1)];

fn drop_sand(map: &mut CharMap, mut pos: Pos2) -> bool {
  while pos.y < map.dims().y - 1 {
    assert!(pos.x >= 0 && pos.x < map.dims().x);
    if let Some(d) = DIRS.iter().find(|d| map[pos + *d] == b'.') {
      pos += *d;
    } else {
      map[pos] = b'o';
      return true;
    }
  }
  assert!(pos.x >= 0 && pos.x < map.dims().x, "{} {}", pos, map.dims());
  map[pos] = b'o';
  false
}

fn solve(path: &str) -> (usize, usize) {
  let input = input_data(14, path);
  let lines = input
    .lines()
    .map(|line| line.split("->").map(to_pos).collect::<Vec<_>>())
    .collect::<Vec<Vec<_>>>();

  let positions = lines.iter().flat_map(|line| line.iter());
  let floor_y = positions.clone().map(|p| p.y).max().unwrap() + 2;
  let min_x = 500 - floor_y;
  let max_x = 500 + floor_y;

  assert!(min_x >= 0);
  let mut map = CharMap::from_dims(Pos2::new(max_x, floor_y), b'.');

  lines.iter()
    .flat_map(|line| line.as_slice().windows(2))
    .flat_map(|line| line[0].line_to(line[1]))
    .for_each(|p| map[p] = b'#');

  let mut total = 0;
  while drop_sand(&mut map, Pos2::new(500, 0)) {
    total += 1;
  }
  let first = total;

  // Count the last dropped
  total += 1;
  while map[Pos2::new(500, 0)] != b'o' {
    drop_sand(&mut map, Pos2::new(500, 0));
    total += 1;
  }
  (first, total)
}

#[test]
fn test() {
  assert_eq!((24, 93), solve("test.txt"));
  assert_eq!((618, 26358), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("test.txt: {} and {}", input.0, input.1);
}
