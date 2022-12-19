use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Pos3 {
  pub x: isize,
  pub y: isize,
  pub z: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Dir3 {
  pub dx: isize,
  pub dy: isize,
  pub dz: isize,
}

impl Pos3 {
  pub const fn new(x: isize, y: isize, z: isize) -> Pos3 {
    Pos3 { x, y, z }
  }

  pub const fn zero() -> Pos3 {
    Pos3 { x: 0, y: 0, z: 0 }
  }

  /// Check if position is inside the rect. Both bounds are inclusive
  pub fn inside_rect(&self, low: Pos3, high: Pos3) -> bool {
    self.x >= low.x && self.x <= high.x && self.y >= low.y && self.y <= high.y && self.z >= low.z && self.z <= high.z
  }
}

impl Dir3 {
  pub const fn new(dx: isize, dy: isize, dz: isize) -> Dir3 {
    Dir3 { dx, dy, dz }
  }
  /// All 6 cardinal directions
  pub fn all_6() -> impl Iterator<Item = Dir3> {
    [(1, 0, 0), (0, 1, 0), (-1, 0, 0), (0, -1, 0), (0, 0, -1), (0, 0, 1)]
      .into_iter()
      .map(|(dx, dy, dz)| Dir3::new(dx, dy, dz))
  }
}

impl AddAssign<Dir3> for Pos3 {
  fn add_assign(&mut self, rhs: Dir3) {
    self.x += rhs.dx;
    self.y += rhs.dy;
    self.z += rhs.dz;
  }
}

impl Add<Dir3> for Pos3 {
  type Output = Pos3;
  fn add(mut self, rhs: Dir3) -> Pos3 {
    self += rhs;
    self
  }
}

impl SubAssign<Pos3> for Pos3 {
  fn sub_assign(&mut self, rhs: Pos3) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl Sub<Pos3> for Pos3 {
  type Output = Pos3;
  fn sub(mut self, rhs: Pos3) -> Pos3 {
    self -= rhs;
    self
  }
}

impl<'a> Add<&'a Dir3> for Pos3 {
  type Output = Pos3;
  fn add(self, rhs: &'a Dir3) -> Pos3 {
    self + *rhs
  }
}

impl std::fmt::Display for Pos3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl FromStr for Pos3 {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut it = s.split(",");
    Ok(Pos3 {
      x: it.next().unwrap().parse().unwrap(),
      y: it.next().unwrap().parse().unwrap(),
      z: it.next().unwrap().parse().unwrap(),
    })
  }
}
