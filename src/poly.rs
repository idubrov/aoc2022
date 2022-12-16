use crate::Pos2;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Poly {
  pub points: Vec<Pos2>,
}

impl std::fmt::Display for Poly {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[")?;
    for point in &self.points {
      write!(f, "{} => ", point)?;
    }
    writeln!(f, "{}]", self.points[0])
  }
}

impl Poly {
  pub fn new(points: Vec<Pos2>) -> Poly {
    Poly { points }
  }

  pub fn is_inside(&self, _point: Pos2) -> bool {
    unimplemented!()
  }

  pub fn intersect(&self, _other: &Poly) -> Poly {
    unimplemented!()
  }
}
