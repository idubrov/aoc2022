use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Pos2 {
  pub x: isize,
  pub y: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Dir2 {
  pub dx: isize,
  pub dy: isize,
}

impl Pos2 {
  pub const fn new(x: isize, y: isize) -> Pos2 {
    Pos2 { x, y }
  }

  pub const fn zero() -> Pos2 {
    Pos2 { x: 0, y: 0 }
  }

  /// Check if position is inside the rect. Both bounds are inclusive
  pub fn inside_rect(&self, low: Pos2, high: Pos2) -> bool {
    self.x >= low.x && self.x <= high.x && self.y >= low.y && self.y <= high.y
  }

  /// Iterate all positions (left to right, then top to bottom) for the rect. Both bounds
  /// are inclusive
  pub fn iter_rect(low: Pos2, high: Pos2) -> impl Iterator<Item = Pos2> {
    Pos2RectIterator {
      current: low,
      top_left: low,
      bottom_right: high,
    }
  }

  /// Cast from a given position in a given direction
  pub fn cast_ray(mut self, dir: Dir2) -> impl Iterator<Item = Pos2> {
    std::iter::from_fn(move || {
      let res = self;
      self += dir;
      Some(res)
    })
  }

  pub fn line_to(self, end: Pos2) -> impl Iterator<Item = Pos2> {
    let (from, to) = if self.x == end.x {
      (self.y.min(end.y), self.y.max(end.y))
    } else if self.y == end.y {
      (self.x.min(end.x), self.x.max(end.x))
    } else {
      panic!("must be horizontal or vertical")
    };

    (from..=to).map(move |i| {
      if self.x == end.x {
        Pos2::new(self.x, i)
      } else {
        Pos2::new(i, self.y)
      }
    })
  }
}

impl Dir2 {
  pub const fn new(dx: isize, dy: isize) -> Dir2 {
    Dir2 { dx, dy }
  }

  /// All 8 cardinal directions
  pub fn all_8() -> impl Iterator<Item = Dir2> {
    (-1..=1).flat_map(move |dy| {
      (-1..=1)
        .filter(move |dx| (*dx != 0 || dy != 0))
        .map(move |dx| Dir2::new(dx, dy))
    })
  }

  /// All 4 cardinal directions
  pub fn all_4() -> impl Iterator<Item = Dir2> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
      .into_iter()
      .map(|(dx, dy)| Dir2::new(dx, dy))
  }
}

impl AddAssign<Dir2> for Pos2 {
  fn add_assign(&mut self, rhs: Dir2) {
    self.x += rhs.dx;
    self.y += rhs.dy;
  }
}

impl Add<Dir2> for Pos2 {
  type Output = Pos2;
  fn add(mut self, rhs: Dir2) -> Pos2 {
    self += rhs;
    self
  }
}

impl SubAssign<Pos2> for Pos2 {
  fn sub_assign(&mut self, rhs: Pos2) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

impl Sub<Pos2> for Pos2 {
  type Output = Pos2;
  fn sub(mut self, rhs: Pos2) -> Pos2 {
    self -= rhs;
    self
  }
}

impl<'a> Add<&'a Dir2> for Pos2 {
  type Output = Pos2;
  fn add(self, rhs: &'a Dir2) -> Pos2 {
    self + *rhs
  }
}

pub struct Pos2RectIterator {
  current: Pos2,
  top_left: Pos2,
  bottom_right: Pos2,
}

impl Iterator for Pos2RectIterator {
  type Item = Pos2;

  fn next(&mut self) -> Option<Self::Item> {
    if !self.current.inside_rect(self.top_left, self.bottom_right) {
      return None;
    }
    let item = self.current;
    self.current.x += 1;
    if self.current.x >= self.bottom_right.x {
      self.current.x = self.top_left.x;
      self.current.y += 1;
    }
    Some(item)
  }
}

impl std::fmt::Display for Pos2 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
