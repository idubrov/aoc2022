use std::collections::HashSet;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
  x: i32,
  y: i32,
}
struct Rope<const N: usize> {
  knots: [Pos; N],
}

impl <const N: usize> Rope<N> {
  fn advance(&mut self) {

    for idx in 0..N - 1 {
      let ddx = self.knots[idx].x - self.knots[idx + 1].x;
      let ddy = self.knots[idx].y - self.knots[idx + 1].y;
      let ex = ddx.abs();
      let ey = ddy.abs();
      if ex >= 2 {
        if ey == 0 {
          self.knots[idx + 1].x += ddx / ex;
        } else {
          self.knots[idx + 1].x += ddx / ex;
          self.knots[idx + 1].y += ddy / ey;
        }
      } else if ey >= 2 {
        if ex == 0 {
          self.knots[idx + 1].y += ddy / ey;
        } else {
          self.knots[idx + 1].x += ddx / ex;
          self.knots[idx + 1].y += ddy / ey;
        }
      }
    }
  }
}

fn solve<const N: usize>(input: &str) -> usize {
  let mut rope = Rope {
    knots: [Pos::default(); N]
  };
  let mut tails = HashSet::new();
  for line in input.lines() {
    let mut it = line.splitn(2, " ");
    let cmd = it.next().unwrap();
    let dist = it.next().unwrap();
    let (dx, dy) = match cmd {
      "R" => (1, 0),
      "U" => (0, 1),
      "L" => (-1, 0),
      "D" => (0, -1),
      _ => unreachable!(),
    };

    for _ in 0..dist.parse::<usize>().unwrap() {
      rope.knots[0].x += dx;
      rope.knots[0].y += dy;
      rope.advance();
      tails.insert(*rope.knots.last().unwrap());
    }
  }
  tails.len()
}

fn main() {
  let input = std::fs::read_to_string("p09/src/input.txt").unwrap();
  eprintln!("{}", solve::<2>(&input));
  eprintln!("{}", solve::<10>(&input));
}
