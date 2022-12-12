use aoc2022::*;

fn range(s: &str) -> (usize, usize) {
  let mut it = s.split("-");
  (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
}

fn main() {
  let input = input_data(4, "input.txt");
  let mut total = 0;
  for line in input.split("\n") {
    let mut it = line.split(",");
    let (x0, x1) = range(it.next().unwrap());
    let (y0, y1) = range(it.next().unwrap());
    if (x0 <= y0 && x1 >= y1) || (y0 <= x0 && y1 >= x1) {
      total += 1;
    }
  }
  eprintln!("{}", total);

  let input = input_data(4, "input.txt");
  let mut total = 0;
  for line in input.split("\n") {
    let mut it = line.split(",");
    let (x0, x1) = range(it.next().unwrap());
    let (y0, y1) = range(it.next().unwrap());
    if !(x1 < y0 || y1 < x0) {
      total += 1;
    }
  }
  eprintln!("{}", total);
}
