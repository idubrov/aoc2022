use aoc2022::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

static RE: Lazy<Regex> =
  Lazy::new(|| Regex::new("^Valve ([A-Z]+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)$").unwrap());

#[derive(Debug, Clone)]
struct Valve {
  rate: isize,
  tunnels: Vec<usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct VisitState {
  pos: usize,
  open: usize,
  time: isize,
}

impl VisitState {
  pub fn new(pos: usize) -> VisitState {
    VisitState { pos, open: 0, time: 0 }
  }
}

impl Ord for VisitState {
  fn cmp(&self, other: &Self) -> Ordering {
    other
      .time
      .cmp(&self.time)
      .then_with(|| self.pos.cmp(&other.pos))
      .then_with(|| self.open.cmp(&other.open))
  }
}

impl PartialOrd for VisitState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Valve {
  fn build_dict<'s>(line: &'s str, dict: &mut HashMap<&'s str, usize>) {
    let captures = RE.captures(line).unwrap();
    let mut it = captures.iter().skip(1);
    let valve = it.next().unwrap().unwrap().as_str().try_into().unwrap();
    let idx = dict.len();
    dict.insert(valve, idx);
  }
  fn from_str(line: &str, dict: &HashMap<&str, usize>) -> Valve {
    let captures = RE.captures(line).unwrap();
    let mut it = captures.iter().skip(2);
    let rate = it.next().unwrap().unwrap().as_str().parse::<isize>().unwrap();
    let tunnels = it
      .next()
      .unwrap()
      .unwrap()
      .as_str()
      .split(", ")
      .map(|t| dict[t])
      .collect::<Vec<_>>();
    Valve { rate, tunnels }
  }
}

fn rates(infos: &[Valve], open: usize) -> isize {
  (0..infos.len())
    .filter(|idx| ((1 << idx) & open) != 0)
    .map(|idx| infos[idx].rate)
    .sum::<isize>()
}

fn find_path(
  infos: &[Valve],
  dists: &Vec<Vec<Option<isize>>>,
  init: VisitState,
  finish: isize,
) -> HashMap<usize, isize> {
  // We never open the initial state.
  assert_eq!(infos[init.pos].rate, 0);

  let mut scores = HashMap::new();
  let mut max = HashMap::new();
  let mut queue: BinaryHeap<(isize, VisitState)> = BinaryHeap::new();
  queue.push((0, init));
  scores.insert(init, 0);
  while let Some((score, state)) = queue.pop() {
    let rate = rates(infos, state.open);
    let max_entry: &mut isize = max.entry(state.open).or_default();
    *max_entry = (*max_entry).max(score + (finish - state.time) * rate);

    for (valve, info) in infos.iter().enumerate() {
      let dt = dists[state.pos][valve].unwrap_or(finish) + 1;
      if info.rate == 0 || (state.open & (1 << valve)) != 0 || state.time + dt > finish {
        continue;
      }

      let next = VisitState {
        pos: valve,
        open: state.open | (1 << valve),
        time: state.time + dt,
      };

      let next_score = score + rate * dt;
      if scores.get(&next).copied().unwrap_or(isize::MIN) < next_score {
        scores.insert(next, next_score);
        queue.push((next_score, next));
      }
    }
  }
  max
}

fn solve(path: &str) -> (isize, isize) {
  let input = input_data(16, path);
  let mut dict = HashMap::new();
  input.lines().for_each(|line| Valve::build_dict(line, &mut dict));
  let infos = input
    .lines()
    .map(|line| Valve::from_str(line, &dict))
    .collect::<Vec<_>>();

  let dists = calculate_shortest_dists(&infos);

  let all = find_path(&infos, &dists, VisitState::new(dict["AA"]), 30);
  let first = *all.iter().map(|(_, score)| score).max().unwrap();

  let myself_scores = find_path(&infos, &dists, VisitState::new(dict["AA"]), 26);
  let mut second = 0;
  for (open, myself_score) in myself_scores {
    let second_init = VisitState {
      open,
      ..VisitState::new(dict["AA"])
    };
    let elephant_scores = find_path(&infos, &dists, second_init, 26);
    let elephant_score = *elephant_scores.iter().map(|(_, s)| s).max().unwrap();
    second = second.max(elephant_score + myself_score - 26 * rates(&infos, open));
  }
  (first, second)
}

fn calculate_shortest_dists(infos: &Vec<Valve>) -> Vec<Vec<Option<isize>>> {
  let mut dists = vec![vec![None; infos.len()]; infos.len()];

  for (from, info) in infos.iter().enumerate() {
    for valve in &info.tunnels {
      dists[from][*valve] = Some(1);
    }
    dists[from][from] = Some(0);
  }

  for k in 0..infos.len() {
    for i in 0..infos.len() {
      for j in 0..infos.len() {
        match (dists[i][k], dists[k][j]) {
          (Some(ik), Some(kj)) if dists[i][j].map_or(true, |ij| ij > ik + kj) => {
            dists[i][j] = Some(ik + kj);
          }
          _ => {}
        }
      }
    }
  }
  dists
}

#[test]
fn test() {
  assert_eq!((1651, 1707), solve("test.txt"));
  assert_eq!((1638, 2400), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
