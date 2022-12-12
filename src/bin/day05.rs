use aoc2022::*;

fn main() {
  let input = input_data(5, "input.txt");

  let mut stacks: [Vec<u8>; 9] = [
    b"BPNQHDRT".to_vec(),
    b"WGBJTV".to_vec(),
    b"NRHDSVMQ".to_vec(),
    b"PZNMC".to_vec(),
    b"DZB".to_vec(),
    b"VCWZ".to_vec(),
    b"GZNCVQLS".to_vec(),
    b"LGJMDNV".to_vec(),
    b"TPMFZCG".to_vec(),
  ];

  for line in input.lines() {
    let mut it = line.split(" ");
    it.next();
    let count = it.next().unwrap().parse::<usize>().unwrap();
    it.next();
    let from = it.next().unwrap().parse::<usize>().unwrap() - 1;
    it.next();
    let to = it.next().unwrap().parse::<usize>().unwrap() - 1;
    for _ in 0..count {
      let x = stacks[from].pop().unwrap();
      stacks[to].push(x);
    }
  }
  for idx in 0..9 {
    eprint!("{}", *stacks[idx].last().unwrap() as char);
  }
  eprintln!();

  let mut stacks: [Vec<u8>; 9] = [
    b"BPNQHDRT".to_vec(),
    b"WGBJTV".to_vec(),
    b"NRHDSVMQ".to_vec(),
    b"PZNMC".to_vec(),
    b"DZB".to_vec(),
    b"VCWZ".to_vec(),
    b"GZNCVQLS".to_vec(),
    b"LGJMDNV".to_vec(),
    b"TPMFZCG".to_vec(),
  ];

  for line in input.lines() {
    let mut it = line.split(" ");
    it.next();
    let count = it.next().unwrap().parse::<usize>().unwrap();
    it.next();
    let from = it.next().unwrap().parse::<usize>().unwrap() - 1;
    it.next();
    let to = it.next().unwrap().parse::<usize>().unwrap() - 1;
    let idx_from = &stacks[from].len() - count;
    let taken = stacks[from].drain(idx_from..).into_iter().collect::<Vec<u8>>();
    stacks[to].extend(taken);
  }
  for idx in 0..9 {
    eprint!("{}", *stacks[idx].last().unwrap() as char);
  }
  eprintln!();
}
