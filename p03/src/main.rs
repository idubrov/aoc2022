use std::collections::HashSet;

fn idx(ch: u8) -> usize {
  if ch >= b'a' && ch <= b'z' {
    usize::from(ch - b'a')
  } else {
    usize::from(ch - b'A' + 26)
  }
}

fn main() {
  let total: usize = std::fs::read_to_string("p03/src/input.txt").unwrap().split("\n")
    .map(|line| {
      let mut flags = [0; 52];
      let line = line.trim().as_bytes();
      for item in line[..line.len() / 2].iter() {
        flags[idx(*item)] |= 1;
      }
      for item in line[line.len() / 2..].iter() {
        flags[idx(*item)] |= 2;
      }
      flags.iter().position(|item| *item == 3).unwrap() + 1
    }).sum();
  eprintln!("{}", total);


  let lines = std::fs::read_to_string("p03/src/input.txt").unwrap();
  let lines = lines
    .split("\n")
    .collect::<Vec<&str>>();

  let total2: usize = lines.chunks(3)
    .map(|chunks| {
      let mut flags = [0; 52];
      for chunk in 0..3 {
        for item in chunks[chunk].as_bytes().iter().copied() {
          flags[idx(item)] |= 1 << chunk;
        }
      }
      flags.iter().position(|item| *item == 7).unwrap() + 1
    })
    .sum();
  eprintln!("{}", total2);
}
