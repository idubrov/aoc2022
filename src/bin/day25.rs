use aoc2022::*;

fn from_snafu(s: &str) -> isize {
  let mut result = 0;
  for ch in s.chars() {
    let digit = match ch {
      '0' => 0,
      '1' => 1,
      '2' => 2,
      '-' => -1,
      '=' => -2,
      _ => unreachable!(),
    };
    result *= 5;
    result += digit;
  }
  result
}

fn to_snafu(mut num: isize) -> String {
  let mut result = String::new();
  while num != 0 {
    match num % 5 {
      0 => result.push('0'),
      1 => result.push('1'),
      2 => result.push('2'),
      3 => {
        num += 5;
        result.push('=')
      }
      4 => {
        num += 5;
        result.push('-');
      }
      _ => unreachable!(),
    }
    num /= 5;
  }
  result.chars().rev().collect::<String>()
}

fn solve(path: &str) -> String {
  let input = input_data(25, path);
  let nums = input.lines().map(from_snafu).collect::<Vec<_>>();
  let sum = nums.iter().sum::<isize>();
  to_snafu(sum)
}

#[test]
fn test() {
  assert_eq!("2=-1=0".to_owned(), solve("test.txt"));
  assert_eq!("2=000=22-0-102=-1001".to_owned(), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {}", test);

  let input = solve("input.txt");
  println!("input.txt: {}", input);
}
