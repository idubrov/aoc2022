use aoc2022::*;

#[derive(Debug, Default)]
pub struct Visibility {
  l: bool,
  r: bool,
  t: bool,
  b: bool,
}

fn scan(grid: &[Vec<i32>], mut x: isize, mut y: isize, dx: isize, dy: isize) -> i32 {
  let start = grid[y as usize][x as usize];
  let mut lowest = 0;
  x += dx;
  y += dy;
  let mut count = 0;
  while y >= 0 && x >= 0 && y < grid.len() as isize && x < grid[0].len() as isize {
    lowest = lowest.max(grid[y as usize][x as usize]);
    count += 1;
    if grid[y as usize][x as usize] >= start {
      break;
    }
    x += dx;
    y += dy;
  }
  count
}

fn main() {
  let input = input_data(8, "input.txt");
  let grid = input
    .lines()
    .map(|line| line.as_bytes().iter().map(|ch| (ch - b'0') as i32).collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let mut visibility = Vec::new();
  for y in 0..grid.len() {
    let mut row = Vec::new();
    for _ in 0..grid[y].len() {
      row.push(Visibility::default());
    }
    visibility.push(row);
  }

  for y in 0..grid.len() {
    let mut highest = -1;
    for x in 0..grid[y].len() {
      if grid[y][x] > highest {
        highest = grid[y][x];
        visibility[y][x].l = true;
      }
    }
    highest = -1;
    for x in (0..grid[y].len()).rev() {
      if grid[y][x] > highest {
        highest = grid[y][x];
        visibility[y][x].r = true;
      }
    }
  }

  for x in 0..grid[0].len() {
    let mut highest = -1;
    for y in 0..grid.len() {
      if grid[y][x] > highest {
        highest = grid[y][x];
        visibility[y][x].t = true;
      }
    }
    highest = -1;
    for y in (0..grid.len()).rev() {
      if grid[y][x] > highest {
        highest = grid[y][x];
        visibility[y][x].b = true;
      }
    }
  }

  let mut total = 0;
  for y in 0..grid.len() {
    for x in 0..grid[y].len() {
      let v = &visibility[y][x];
      if v.l || v.r || v.t || v.b {
        total += 1;
      }
    }
  }

  let mut best_score = 0;
  for y in 0..grid.len() {
    for x in 0..grid[y].len() {
      let y = y as isize;
      let x = x as isize;
      let score =
        scan(&grid, x, y, 0, -1) * scan(&grid, x, y, -1, 0) * scan(&grid, x, y, 1, 0) * scan(&grid, x, y, 0, 1);
      if score > best_score {
        best_score = score;
      }
    }
  }
  eprintln!("{}", total);
  eprintln!("{}", best_score);
}
