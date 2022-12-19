use aoc2022::*;

fn solve(path: &str) -> (usize, usize) {
  let coords = input_data(18, path)
    .lines()
    .map(|x| x.parse::<Pos3>().unwrap())
    .collect::<Vec<_>>();

  let low = Pos3::new(-1, -1, -1);
  let high = Pos3 {
    x: coords.iter().map(|c| c.x).max().unwrap() + 1,
    y: coords.iter().map(|c| c.y).max().unwrap() + 1,
    z: coords.iter().map(|c| c.z).max().unwrap() + 1,
  };
  let dims = high - low + Dir3::new(1, 1, 1);
  let mut map = vec![0u8; (dims.x * dims.y * dims.z) as usize];
  for pos in &coords {
    let pos2 = *pos - low;
    map[(pos2.z * dims.x * dims.y + pos2.y * dims.x + pos2.x) as usize] = 1;
  }

  let mut queue = vec![low];
  while let Some(pos) = queue.pop() {
    let pos2 = pos - low;
    map[(pos2.z * dims.x * dims.y + pos2.y * dims.x + pos2.x) as usize] = 2;
    queue.extend(
      Dir3::all_6()
        .map(|d| pos + d)
        .filter(|next| next.inside_rect(low, high))
        .filter(|next| {
          let next = *next - low;
          map[(next.z * dims.x * dims.y + next.y * dims.x + next.x) as usize] == 0
        }),
    );
  }

  let first = coords
    .iter()
    .map(|pos| {
      Dir3::all_6()
        .filter(|d| {
          let next = *pos + *d;
          !next.inside_rect(low, high) || {
            let next = next - low;
            map[(next.z * dims.x * dims.y + next.y * dims.x + next.x) as usize] != 1
          }
        })
        .count()
    })
    .sum::<usize>();
  let second = coords
    .iter()
    .map(|pos| {
      Dir3::all_6()
        .filter(|d| {
          let next = *pos + *d;
          !next.inside_rect(low, high) || {
            let next = next - low;
            map[(next.z * dims.x * dims.y + next.y * dims.x + next.x) as usize] == 2
          }
        })
        .count()
    })
    .sum::<usize>();
  (first, second)
}

#[test]
fn test() {
  assert_eq!((64, 58), solve("test.txt"));
  assert_eq!((4400, 2522), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
