use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use aoc2022::visualize::Channel;
use aoc2022::*;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new("Sensor at x=(-?\\d+)+, y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)").unwrap());

#[derive(Debug, Clone, Copy)]
struct Info {
  sensor: Pos2,
  beacon: Pos2,
}

impl FromStr for Info {
  type Err = ();

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let mut captures = RE.captures(line).unwrap();
    let mut it = captures.iter().skip(1).map(|s| s.unwrap().as_str().parse::<isize>().unwrap());
    Ok(Info {
      sensor: Pos2::new(it.next().unwrap(), it.next().unwrap()),
      beacon: Pos2::new(it.next().unwrap(), it.next().unwrap()),
    })
  }
}

impl Info {
  fn distance(&self) -> isize {
    let Pos2 { x, y } = self.beacon - self.sensor;
    x.abs() + y.abs()
  }
  fn collect_ranges(&self, result: &mut Vec<(isize, isize)>, row: isize, beacon: bool) {
    let y_dist = (self.sensor.y - row).abs();
    let x_dist = self.distance() - y_dist;
    if beacon && self.beacon.y == row {
      if x_dist == 0 {
        // The tip
      } else if self.beacon.x < self.sensor.x {
        result.push((self.beacon.x + 1, self.sensor.x + x_dist))
      } else {
        result.push((self.sensor.x - x_dist, self.beacon.x - 1))
      }
    } else if x_dist >= 0 {
      result.push((self.sensor.x - x_dist, self.sensor.x + x_dist));
    }
  }
}

fn find_gap(mut ranges: &[(isize, isize)]) -> Option<isize> {
  let mut it = ranges.iter().copied();
  let (mut from, mut to) = (-1, -1);
  while let Some((nf, nt)) = it.next() {
    if nt <= to {
      continue;
    } else if nf <= to {
      to = nt;
    } else {
      if nf == to + 2 {
        return Some(to + 1);
      }
      from = nf;
      to = nt;
    }
  }
  None
}

fn count_misses(ranges: &[(isize, isize)]) -> isize {
  let mut total = 0;
  let mut it = ranges.iter().copied();
  let (mut from, mut to) = it.next().unwrap();
  while let Some((nf, nt)) = it.next() {
    if nt <= to {
      continue;
    } else if nf <= to {
      to = nt;
    } else {
      total += (to - from) + 1;
      from = nf;
      to = nt;
    }
  }
  total += (to - from) + 1;
  total
}

fn collect_all(infos: &[Info], row: isize, beacon: bool, mut ranges: &mut Vec<(isize, isize)>) {
  ranges.clear();
  for info in infos {
    info.collect_ranges(&mut ranges, row, beacon);
  }
  ranges.sort_by_key(|(f, _)| *f);
}

fn solve(path: &str, row: isize, range: isize) -> (isize, isize) {
  let input = input_data(15, path);
  let infos = input.lines().map(|line| line.parse::<Info>().unwrap()).collect::<Vec<_>>();

  let mut ranges = Vec::new();
  collect_all(&infos, row, true, &mut ranges);

  let first = count_misses(&ranges);

  let mut second = None;
  'outer: for info in &infos {
    let rim = info.distance() + 1;
    let dirs = [
      Dir2::new(1, 1),
      Dir2::new(-1, 1),
      Dir2::new(-1, -1),
      Dir2::new(1, -1),
    ];
    let start = Pos2::new(info.sensor.x, info.sensor.y - rim);
    let mut pos = start;

    for dir in dirs {
      for _ in 0..rim {
        if pos.x >= 0 && pos.x <= range && pos.y >= 0 && pos.y <= range {
          if infos.iter().all(|other| (other.sensor - pos).manhattan() > other.distance()) {
            second = Some(pos.x * 4000000 + pos.y);
            break 'outer;
          }
        }
        pos += dir;
      }
    }
    assert_eq!(start, pos);
  }
  (first, second.unwrap())
}


#[test]
fn test() {
  assert_eq!((26, 56000011), solve("test.txt", 10, 20));
  assert_eq!((6124805, 12555527364986), solve("input.txt", 2000000, 4000000));
}

fn main() {
  let test = solve("test.txt", 10, 20);
  println!("test.txt: {} and {}", test.0, test.1);

  // let input = solve("input.txt", 2000000, 4000000);
  // println!("input.txt: {} and {}", input.0, input.1);
}
