use aoc2022::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

static RE: Lazy<Regex> =
  Lazy::new(|| Regex::new("Sensor at x=(-?\\d+)+, y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)").unwrap());

#[derive(Debug, Clone, Copy)]
struct Info {
  sensor: Pos2,
  beacon: Pos2,
}

impl FromStr for Info {
  type Err = ();

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let captures = RE.captures(line).unwrap();
    let mut it = captures
      .iter()
      .skip(1)
      .map(|s| s.unwrap().as_str().parse::<isize>().unwrap());
    Ok(Info {
      sensor: Pos2::new(it.next().unwrap(), it.next().unwrap()),
      beacon: Pos2::new(it.next().unwrap(), it.next().unwrap()),
    })
  }
}

impl Info {
  fn distance(&self) -> isize {
    // FIXME: manhattan!
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

  fn u_v_allowed(&self) -> [Area; 4] {
    let uv = self.sensor.to_uv();
    let d = self.distance();
    [
      Area::left_of(uv.x - d - 1),
      Area::right_of(uv.x + d + 1),
      Area::top_of(uv.y - d - 1),
      Area::bottom_of(uv.y + d + 1),
    ]
  }
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
  let infos = input
    .lines()
    .map(|line| line.parse::<Info>().unwrap())
    .collect::<Vec<_>>();

  let mut ranges = Vec::new();
  collect_all(&infos, row, true, &mut ranges);

  let first = count_misses(&ranges);

  let mut areas = HashSet::new();
  // UV square that includes our target XY square
  areas.insert(Area::new(Pos2::new(0, -range), Pos2::new(2 * range, range)));
  for info in &infos {
    areas = areas
      .iter()
      .flat_map(|area| info.u_v_allowed().into_iter().map(|other| area.intersect(&other)))
      .filter_map(|x| x)
      .collect::<HashSet<Area>>();
  }
  let mut second = None;
  'outer: for area in areas {
    // The solution must be one of the corners in the UV. A better approach would be to intersect
    // all our UV areas with the target XY area, but haven't figured out simple solution yet.
    for corner in area.corners() {
      if (corner.x + corner.y) % 2 == 0 {
        let candidate = corner.from_uv();
        if candidate.inside_rect(Pos2::new(0, 0), Pos2::new(range, range)) {
          second = Some(candidate.x * 4000000 + candidate.y);
          break 'outer;
        }
      }
    }
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

  let input = solve("input.txt", 2000000, 4000000);
  println!("input.txt: {} and {}", input.0, input.1);
}
