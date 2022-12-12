use aoc2022::*;
use std::collections::HashMap;

struct Machine {
  cycle: i32,
  x: i32,
  screen: Vec<bool>,
}

impl Machine {
  fn init() -> Self {
    Machine {
      cycle: 0,
      x: 1,
      screen: vec![false; 40 * 6],
    }
  }

  fn advance(&mut self, instr: i32) {
    let ray_pos = self.cycle % 40;
    if (self.x - ray_pos).abs() <= 1 {
      self.screen[(self.cycle % 240) as usize] = true;
    }
    self.x += instr;
    self.cycle += 1;
  }

  fn render(&self) {
    for y in 0..6 {
      for x in 0..40 {
        if self.screen[y * 40 + x] {
          eprint!("#");
        } else {
          eprint!(".");
        }
      }
      eprintln!();
    }
  }
}

fn solve(path: &str) {
  let input = input_data(10, path);
  let mut processed = Vec::new();
  for instr in input.lines() {
    processed.push(0);
    if instr.starts_with("addx ") {
      processed.push(instr[5..].parse::<i32>().unwrap());
    }
  }
  let mut machine = Machine::init();
  let mut state = HashMap::new();
  for instr in processed {
    machine.advance(instr);
    state.insert(machine.cycle + 1, machine.x);
  }
  let cycles = [20, 60, 100, 140, 180, 220];
  let total: i32 = cycles.iter().map(|c| state[&c] * c).sum();
  eprintln!("{}: {}", path, total);
  machine.render();
}

fn main() {
  solve("test.txt");
  solve("input.txt");
}
