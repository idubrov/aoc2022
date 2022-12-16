use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use once_cell::sync::Lazy;
use regex::Regex;
use aoc2022::*;

static RE: Lazy<Regex> =
  Lazy::new(|| Regex::new("^Valve ([A-Z]+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)$").unwrap());

#[derive(Debug, Clone)]
struct Valve {
  rate: isize,
  tunnels: Vec<usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct ScoreState {
  pos: usize,
  open: usize,
  time: isize,
  score: isize,
}

impl Ord for ScoreState {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score.cmp(&other.score)
      .then_with(|| other.time.cmp(&self.time))
      .then_with(|| self.pos.cmp(&other.pos))
      .then_with(|| self.open.cmp(&other.open))
  }
}

impl PartialOrd for ScoreState {
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
    let tunnels = it.next().unwrap().unwrap().as_str().split(", ").map(|t| dict[t]).collect::<Vec<_>>();
    Valve {
      rate, tunnels,
    }
  }
}

fn rates(infos: &[Valve], open: usize) -> isize {
  (0..infos.len()).filter(|idx| ((1 << idx) & open) != 0).map(|idx| infos[idx].rate).sum::<isize>()
}

fn next_states<'a>(infos: &'a [Valve], dists: &'a Vec<Vec<Option<isize>>>, state: ScoreState) -> impl Iterator<Item =ScoreState> + 'a {
  let mut open = None;
  let rate = rates(infos, state.open);
  if infos[state.pos].rate != 0 && (state.open & (1 << state.pos)) == 0 {
    open = Some(ScoreState {
      pos: state.pos,
      open: state.open | (1 << state.pos),
      time: state.time + 1,
      score: state.score + rate,
    });
  }

  open.into_iter().chain(
    infos.iter()
    .enumerate()
    .filter(move |(valve, info)| info.rate != 0 && (state.open & (1 << valve)) == 0 && dists[state.pos][*valve].is_some())
    .map(move |(valve, _)| {
      let delta = dists[state.pos][valve].unwrap() + 1;
      ScoreState {
        pos: valve,
        open: state.open | (1 << valve),
        time: state.time + delta,
        score: state.score + rate * delta,
      }
    })
  )
}

fn bfs(infos: &[Valve], dists: &Vec<Vec<Option<isize>>>, init: ScoreState, finish: isize) -> HashMap<usize, isize> {
  let mut scores = HashMap::new();
  let mut max = HashMap::new();
  let mut queue = BinaryHeap::new();
  queue.push(init);
  scores.insert(init, 0);
  while let Some(state) = queue.pop() {
    let rate = rates(infos, state.open);
    let m: &mut isize = max.entry(state.open).or_default();
    *m = (*m).max(state.score + (finish - state.time) * rate);

    for next in next_states(infos, &dists, state) {
      if next.time <= finish && scores.get(&next).copied().unwrap_or(isize::MIN) < next.score {
        scores.insert(next, next.score);
        queue.push(next);
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

  let init = ScoreState {
    pos: dict["AA"],
    open: 0,
    time: 0,
    score: 0,
  };

  let all_first = bfs(&infos, &dists, init, 30);
  let first = *all_first.iter().map(|(_, score)| score).max().unwrap();

  let all_second = bfs(&infos, &dists, init, 26);
  let mut second = 0;
  for (open, score) in all_second {
    let second_init = ScoreState {
      pos: dict["AA"],
      open,
      time: 0,
      score: score - 26 * rates(&infos, open),
    };
    let all_second = bfs(&infos, &dists, second_init, 26);
    second = second.max(*all_second.iter().map(|(_, score)| score).max().unwrap());
  }
  (first, second)
}

#[test]
fn test() {
  assert_eq!((1651, 1707), solve("test.txt"));
  assert_eq!((1638, 0), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
