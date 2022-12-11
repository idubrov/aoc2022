use aoc2022::*;

fn main() {
  let mut all = input_data(1, "input.txt")
    .split("\n\n")
    .map(|chunk| {
      chunk
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|c| str::parse::<u32>(c).unwrap())
        .sum()
    })
    .collect::<Vec<u32>>();
  all.sort();
  all.reverse();

  eprintln!("{}", all[0]);
  eprintln!("{}", all[0] + all[1] + all[2]);
}
