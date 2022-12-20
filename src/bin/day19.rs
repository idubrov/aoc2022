use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::{Match, Regex};
use aoc2022::*;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap());

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Copy)]
struct Resources {
  ore: usize,
  clay: usize,
  obsidian: usize,
  geode: usize,
}

impl AddAssign for Resources {
  fn add_assign(&mut self, rhs: Self) {
    self.ore += rhs.ore;
    self.clay += rhs.clay;
    self.obsidian += rhs.obsidian;
    self.geode += rhs.geode;
  }
}

impl Add for Resources {
  type Output = Resources;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl Mul<usize> for Resources {
  type Output = Resources;

  fn mul(mut self, rhs: usize) -> Resources {
    self.ore *= rhs;
    self.clay *= rhs;
    self.obsidian *= rhs;
    self.geode *= rhs;
    self
  }
}

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Copy)]
struct State {
  resources: Resources,
  robots: Resources,
  time: usize,
}



#[derive(Debug)]
struct Blueprint {
  blueprint: usize,
  ore_ore: usize,
  clay_ore: usize,
  obsidian_ore: usize,
  obsidian_clay: usize,
  geode_ore: usize,
  geode_obsidian: usize,
}

fn num<'a>(it: &mut impl Iterator<Item = Option<Match<'a>>>) -> usize {
  it.next().unwrap().unwrap().as_str().parse().unwrap()
}

impl FromStr for Blueprint {
  type Err = ();

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let captures = RE.captures(line).unwrap();
    let mut it = captures.iter().skip(1);
    Ok(Blueprint {
      blueprint: num(&mut it),
      ore_ore: num(&mut it),
      clay_ore: num(&mut it),
      obsidian_ore: num(&mut it),
      obsidian_clay: num(&mut it),
      geode_ore: num(&mut it),
      geode_obsidian: num(&mut it),
    })
  }
}

fn when(resource: usize, prod: usize, req: usize) -> Option<usize> {
  if resource >= req {
    Some(0)
  } else if prod == 0 {
    None
  } else {
    let need = (req - resource + prod - 1) / prod;
    Some(need)
  }
}

fn blueprint(blueprint: &Blueprint, limit: usize) -> usize {
  let mut init = State::default();
  init.robots.ore += 1;

  let max_ore = blueprint.ore_ore
    .max(blueprint.clay_ore)
    .max(blueprint.obsidian_ore)
    .max(blueprint.geode_ore);

  let mut scores = HashMap::new();
  let mut max = 0;
  let mut queue: Vec<State> = Vec::new();
  queue.push(init);
  scores.insert(init, 0);
  while let Some(state) = queue.pop() {
    let candidate = (state.resources + state.robots * (limit - state.time)).geode;
    max = max.max(candidate);

    // Build geode?
    if let Some(t1) = when(state.resources.ore, state.robots.ore, blueprint.geode_ore) {
      if let Some(t2) = when(state.resources.obsidian, state.robots.obsidian, blueprint.geode_obsidian) {
        let ticks = t1.max(t2);
        let mut next = state;
        next.resources += next.robots * (ticks + 1);
        next.time += ticks + 1;

        // Build
        next.resources.ore -= blueprint.geode_ore;
        next.resources.obsidian -= blueprint.geode_obsidian;
        next.robots.geode += 1;
        if next.time <= limit {
          queue.push(next);
          if ticks == 0 {
            continue;
          }
        }
      }
    }

    // Build ore?
    if state.robots.ore < max_ore {
      if let Some(ticks) = when(state.resources.ore, state.robots.ore, blueprint.ore_ore) {
        let mut next = state;
        next.resources += next.robots * (ticks + 1);
        next.time += ticks + 1;

        // Build
        next.resources.ore -= blueprint.ore_ore;
        next.robots.ore += 1;
        if next.time <= limit {
          queue.push(next);
        }
      }
    }

    // Build clay?
    if let Some(ticks) = when(state.resources.ore, state.robots.ore, blueprint.clay_ore) {
      let mut next = state;
      next.resources += next.robots * (ticks + 1);
      next.time += ticks + 1;

      // Build
      next.resources.ore -= blueprint.clay_ore;
      next.robots.clay += 1;
      if next.time <= limit {
        queue.push(next);
      }
    }

    // Build obsidian?
    if let Some(t1) = when(state.resources.ore, state.robots.ore, blueprint.obsidian_ore) {
      if let Some(t2) = when(state.resources.clay, state.robots.clay, blueprint.obsidian_clay) {
        let ticks = t1.max(t2);
        let mut next = state;
        next.resources += next.robots * (ticks + 1);
        next.time += ticks + 1;

        // Build
        next.resources.ore -= blueprint.obsidian_ore;
        next.resources.clay -= blueprint.obsidian_clay;
        next.robots.obsidian += 1;
        if next.time <= limit {
          queue.push(next);
        }
      }
    }
  }
  max
}

fn solve(path: &str) -> (usize, usize) {
  let blueprints = input_data(19, path)
    .lines()
    .map(|x| x.parse::<Blueprint>().unwrap())
    .collect::<Vec<_>>();

  let mut first = 0;
  for b in &blueprints {
    first += blueprint(b, 24) * b.blueprint;
  }

  let mut second = 1;
  for b in &blueprints[0..blueprints.len().min(3)] {
    second *= blueprint(b, 32);
  }
  (first, second)
}

#[test]
fn test() {
  assert_eq!((33, 3472), solve("test.txt"));
  assert_eq!((960, 2040), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
