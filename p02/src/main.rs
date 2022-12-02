fn main() {
  let first_round: u32 = std::fs::read_to_string("p02/src/input.txt")
    .unwrap()
    .split("\n")
    .map(|line| {
      let first = u32::from((line.as_bytes()[0] as u8) - b'A');
      let second = u32::from((line.as_bytes()[2] as u8) - b'X');
      if first == second {
        second + 4
      } else if ((first + 1) % 3) == second {
        second + 7
      } else {
        second + 1
      }
    })
    .sum();

  let second_round: u32 = std::fs::read_to_string("p02/src/input.txt")
    .unwrap()
    .split("\n")
    .map(|line| {
      let first = u32::from((line.as_bytes()[0] as u8) - b'A');
      match u32::from((line.as_bytes()[2] as u8) - b'X') {
        0 => ((first + 2) % 3) + 1,
        1 => first + 4,
        2 => ((first + 1) % 3) + 7,
        _ => unreachable!(),
      }
    })
    .sum();
  eprintln!("{}", first_round);
  eprintln!("{}", second_round);
}
