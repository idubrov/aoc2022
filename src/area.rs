use crate::Pos2;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Area {
  pub top_left: Pos2,
  pub bottom_right: Pos2,
}

impl std::fmt::Display for Area {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[{} => {}]", self.top_left, self.bottom_right)
  }
}

impl Area {
  pub fn new(top_left: Pos2, bottom_right: Pos2) -> Area {
    Area { top_left, bottom_right }
  }

  pub fn left_of(pos: isize) -> Area {
    Area::new(Pos2::new(isize::MIN, isize::MIN), Pos2::new(pos, isize::MAX))
  }

  pub fn top_of(pos: isize) -> Area {
    Area::new(Pos2::new(isize::MIN, isize::MIN), Pos2::new(isize::MAX, pos))
  }

  pub fn right_of(pos: isize) -> Area {
    Area::new(Pos2::new(pos, isize::MIN), Pos2::new(isize::MAX, isize::MAX))
  }

  pub fn bottom_of(pos: isize) -> Area {
    Area::new(Pos2::new(isize::MIN, pos), Pos2::new(isize::MAX, isize::MAX))
  }

  pub fn intersect(&self, other: &Area) -> Option<Area> {
    let x0 = self.top_left.x.max(other.top_left.x);
    let y0 = self.top_left.y.max(other.top_left.y);
    let x1 = self.bottom_right.x.min(other.bottom_right.x);
    let y1 = self.bottom_right.y.min(other.bottom_right.y);
    if x0 <= x1 && y0 <= y1 {
      Some(Area {
        top_left: Pos2::new(x0, y0),
        bottom_right: Pos2::new(x1, y1),
      })
    } else {
      None
    }
  }

  pub fn contains(&self, pos: Pos2) -> bool {
    pos.inside_rect(self.top_left, self.bottom_right)
  }

  pub fn corners(&self) -> impl Iterator<Item = Pos2> {
    let Pos2 { x: x0, y: y0 } = self.top_left;
    let Pos2 { x: x1, y: y1 } = self.bottom_right;
    [
      Pos2::new(x0, y0),
      Pos2::new(x1, y0),
      Pos2::new(x0, y1),
      Pos2::new(x1, y1),
    ]
    .into_iter()
  }
}
