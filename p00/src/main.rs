fn main() {
  let mut all = std::fs::read_to_string("p00/src/input.txt")
    .unwrap()
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
