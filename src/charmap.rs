use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{Index, IndexMut};

use crate::{Dir2, Pos2};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoundsBehavior {
  /// Panic when accessing out of bounds
  Panic,
  /// Writes resize map as necessary, reads return a default value.
  Grow { default: u8 },
  /// Ignore writes, reads return a default value.
  Abyss { default: u8, nothing: u8 },
}

impl BoundsBehavior {
  pub fn grow(ch: u8) -> BoundsBehavior {
    BoundsBehavior::Grow { default: ch }
  }
}

#[derive(Clone)]
pub struct CharMap {
  map: Vec<Vec<u8>>,
  tmp: Vec<Vec<u8>>,
  bounds: BoundsBehavior,
  /// Top left corner of the map (inclusize)
  top_left: Pos2,
  /// Bottom right corner of the map (inclusive)
  bottom_right: Pos2,
  // offset: Pos2,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct PathState {
  pos: Pos2,
  cost: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum VisitKind {
  Consider,
  Visit,
}

impl Ord for PathState {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
  }
}

impl PartialOrd for PathState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Index<Pos2> for CharMap {
  type Output = u8;

  fn index(&self, pos: Pos2) -> &Self::Output {
    if self.is_in_bounds(pos) {
      &self.map[(pos.y - self.top_left.y) as usize][(pos.x - self.top_left.x) as usize]
    } else {
      match self.bounds {
        BoundsBehavior::Panic => panic!("{} is out of bounds", pos),
        BoundsBehavior::Abyss { ref default, .. } => default,
        BoundsBehavior::Grow { ref default } => default,
      }
    }
  }
}

impl IndexMut<Pos2> for CharMap {
  fn index_mut(&mut self, pos: Pos2) -> &mut Self::Output {
    if !self.is_in_bounds(pos) {
      match self.bounds {
        BoundsBehavior::Panic => panic!("{} is out of bounds", pos),
        BoundsBehavior::Abyss { ref mut nothing, .. } => return nothing,
        BoundsBehavior::Grow { default } => {
          if self.bottom_right == Pos2::new(-1, -1) {
            self.top_left = pos;
            self.bottom_right = pos;
            self.map = vec![vec![default; 1]; 1];
            return &mut self.map[0][0];
          }
          if pos.x < self.top_left.x {
            let delta = self.top_left.x - pos.x;
            for line in &mut self.map {
              for _ in 0..delta {
                line.insert(0, default);
              }
            }
            self.top_left.x -= delta;
          }
          if pos.y < self.top_left.y {
            let delta = self.top_left.y - pos.y;
            for _ in 0..delta {
              self.map.insert(0, vec![default; self.dims().x as usize]);
            }
            self.top_left.y -= delta;
          }
          if pos.x > self.bottom_right.x {
            let delta = pos.x - self.bottom_right.x;
            for line in &mut self.map {
              for _ in 0..delta {
                line.push(default);
              }
            }
            self.bottom_right.x += delta;
          }
          if pos.y > self.bottom_right.y {
            let delta = pos.y - self.bottom_right.y;
            for _ in 0..delta {
              self.map.push(vec![default; self.dims().x as usize]);
            }
            self.bottom_right.y += delta;
          }
        }
      }
    }
    &mut self.map[(pos.y - self.top_left.y) as usize][(pos.x - self.top_left.x) as usize]
  }
}

impl CharMap {
  pub fn from_text(text: &str) -> Self {
    let map = text.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
    CharMap {
      bounds: BoundsBehavior::Panic,
      top_left: Pos2::zero(),
      bottom_right: Pos2::new((map[0].len() as isize) - 1, (map.len() as isize) - 1),
      tmp: map.clone(),
      map,
    }
  }

  pub fn empty(bounds: BoundsBehavior) -> Self {
    let map = vec![];
    CharMap {
      tmp: map.clone(),
      map,
      bounds,
      top_left: Pos2::zero(),
      bottom_right: Pos2::new(-1, -1),
    }
  }

  /// Set the "default" value for elements outside of the map bounds.
  pub fn with_bounds(mut self, bounds: BoundsBehavior) -> Self {
    self.bounds = bounds;
    self
  }

  pub fn count_adjacent8(&self, pos: Pos2, ch: u8) -> usize {
    Dir2::all_4().filter(|dir| self[pos + dir] == ch).count()
  }

  pub fn is_in_bounds(&self, pos: Pos2) -> bool {
    pos.inside_rect(self.top_left, self.bottom_right)
  }

  /// Cast a ray in a given direction and find first position matching the condition.
  pub fn cast_find(&self, pos: Pos2, dir: Dir2, match_fn: impl Fn(&Self, Pos2) -> bool) -> Option<Pos2> {
    pos
      .cast_ray(dir)
      .skip(1)
      .take_while(|pos| self.is_in_bounds(*pos))
      .find(|pos| match_fn(self, *pos))
  }

  pub fn step_update(&mut self, update_cb: impl Fn(&CharMap, Pos2) -> u8) -> bool {
    let mut changes = false;
    for pos in self.every_pos() {
      let updated = update_cb(&self, pos);
      if self[pos] != updated {
        changes = true;
      }
      self.tmp[pos.y as usize][pos.x as usize] = updated;
    }
    std::mem::swap(&mut self.map, &mut self.tmp);
    changes
  }

  pub fn count(&self, ch: u8) -> usize {
    self
      .map
      .iter()
      .map(|line| line.iter().filter(|ch2| ch == **ch2).count())
      .sum::<usize>()
  }

  pub fn top_left(&self) -> Pos2 {
    self.top_left
  }

  pub fn bottom_right(&self) -> Pos2 {
    self.bottom_right
  }

  pub fn dims(&self) -> Pos2 {
    self.bottom_right - self.top_left + Dir2::new(1, 1)
  }

  pub fn every_pos(&self) -> impl Iterator<Item = Pos2> {
    Pos2::iter_rect(self.top_left, self.bottom_right)
  }

  pub fn find_path(
    &self,
    start: Pos2,
    target_fn: impl Fn(&Self, Pos2) -> bool,
    cost_fn: impl Fn(&Self, Pos2, Pos2) -> Option<usize>,
  ) -> Option<usize> {
    self.find_path_cb(start, target_fn, cost_fn, |_, _, _, _| {})
  }

  pub fn find_path_cb(
    &self,
    start: Pos2,
    target_fn: impl Fn(&Self, Pos2) -> bool,
    cost_fn: impl Fn(&Self, Pos2, Pos2) -> Option<usize>,
    mut visit_fn: impl FnMut(&CharMap, VisitKind, Pos2, usize),
  ) -> Option<usize> {
    let mut costs = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(PathState { pos: start, cost: 0 });
    costs.insert(start, 0);
    while let Some(PathState { pos, cost }) = queue.pop() {
      visit_fn(self, VisitKind::Visit, pos, cost);

      for dir in Dir2::all_4() {
        let next = pos + dir;
        if !self.is_in_bounds(next) {
          continue;
        }
        if let Some(next_cost) = cost_fn(self, pos, next) {
          if cost + next_cost < costs.get(&next).copied().unwrap_or(usize::MAX) {
            costs.insert(next, cost + next_cost);
            visit_fn(self, VisitKind::Consider, next, cost);
            queue.push(PathState {
              pos: next,
              cost: cost + next_cost,
            });
          }
        }
      }
    }
    self
      .every_pos()
      .filter(|p| target_fn(self, *p))
      .map(|t| costs.get(&t).copied().unwrap_or(usize::MAX))
      .min()
  }
}

impl std::fmt::Display for CharMap {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for y in self.top_left.y..=self.bottom_right.y {
      for x in self.top_left.x..=self.bottom_right.x {
        write!(f, "{}", self[Pos2::new(x, y)] as char)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}
