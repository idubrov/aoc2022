use crate::{Dir2, Pos2};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct CharMap {
  map: Vec<Vec<u8>>,
  tmp: Vec<Vec<u8>>,
  border: u8,
  nothing: u8,
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
    if pos.inside_rect(Pos2::zero(), self.dims()) {
      &self.map[pos.y as usize][pos.x as usize]
    } else {
      &self.border
    }
  }
}

impl IndexMut<Pos2> for CharMap {
  fn index_mut(&mut self, pos: Pos2) -> &mut Self::Output {
    if pos.inside_rect(Pos2::zero(), self.dims()) {
      &mut self.map[pos.y as usize][pos.x as usize]
    } else {
      &mut self.nothing
    }
  }
}

impl CharMap {
  pub fn from_text(text: &str) -> Self {
    let map = text.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
    CharMap {
      tmp: map.clone(),
      map,
      border: 0,
      nothing: 0,
    }
  }

  /// Set the "default" value for elements outside of the map bounds.
  pub fn with_border(mut self, border: u8) -> Self {
    self.border = border;
    self
  }

  pub fn count_adjacent8(&self, pos: Pos2, ch: u8) -> usize {
    Dir2::all_4().filter(|dir| self[pos + dir] == ch).count()
  }

  pub fn is_inside(&self, pos: Pos2) -> bool {
    pos.inside_rect(Pos2::zero(), self.dims())
  }

  /// Cast a ray in a given direction and find first position matching the condition.
  pub fn cast_find(&self, pos: Pos2, dir: Dir2, match_fn: impl Fn(&Self, Pos2) -> bool) -> Option<Pos2> {
    pos
      .cast_ray(dir)
      .skip(1)
      .take_while(|pos| self.is_inside(*pos))
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

  pub fn dims(&self) -> Pos2 {
    Pos2::new(self.map[0].len() as isize, self.map.len() as isize)
  }

  pub fn every_pos(&self) -> impl Iterator<Item = Pos2> {
    Pos2::iter_rect(Pos2::zero(), self.dims())
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
    let dims = self.dims();
    let mut costs = vec![vec![usize::MAX; dims.x as usize]; dims.y as usize];
    let mut queue = BinaryHeap::new();
    queue.push(PathState { pos: start, cost: 0 });
    costs[start.y as usize][start.x as usize] = 0;
    while let Some(PathState { pos, cost }) = queue.pop() {
      visit_fn(self, VisitKind::Visit, pos, cost);

      // if target_fn(self, pos) {
      //   return Some(cost);
      // }
      for dir in Dir2::all_4() {
        let next = pos + dir;
        if !self.is_inside(next) {
          continue;
        }
        if let Some(next_cost) = cost_fn(self, pos, next) {
          if cost + next_cost < costs[next.y as usize][next.x as usize] {
            costs[next.y as usize][next.x as usize] = cost + next_cost;
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
      .map(|t| costs[t.y as usize][t.x as usize])
      .min()
  }
}

impl std::fmt::Display for CharMap {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for line in &self.map {
      for item in line {
        write!(f, "{}", *item as char)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}
