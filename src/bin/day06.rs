use aoc2022::*;
use std::collections::HashSet;

fn main() {
  let input = input_data(6, "input.txt").into_bytes();

  let size = 4;
  for idx in 0..input.len() - size {
    let different = input[idx..idx + size].iter().collect::<HashSet<_>>().len() == size;
    if different {
      eprintln!("{}", idx + size);
      break;
    }
  }

  let size = 14;
  for idx in 0..input.len() - size {
    let different = input[idx..idx + size].iter().collect::<HashSet<_>>().len() == size;
    if different {
      eprintln!("{}", idx + size);
      break;
    }
  }
}
