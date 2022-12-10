use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
  AddX(i32),
  Noop,
}

struct Machine {
  cycle: i32,
  x: i32,
  screen: Vec<bool>,
}

impl Machine {
  fn init(delay: usize) -> Self {
    Machine {
      cycle: 1,
      x: 1,
      screen: vec![false; 40 * 6],
    }
  }

  fn advance(&mut self, instr: &Instruction) {
    let sprite_pos = (self.x - 1) % 40;
    let ray_pos = (self.cycle - 1) % 40;
    if sprite_pos == ray_pos ||
      ((sprite_pos + 1) % 40) == ray_pos ||
      ((sprite_pos + 2) % 40) == ray_pos {
      self.screen[((self.cycle - 1) % 240) as usize] = true;
    }
    if let Instruction::AddX(value) = instr {
      self.x += value;
    }
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

fn parse_instr(inst: &str) -> Instruction {
  if inst == "noop" {
    Instruction::Noop
  } else if inst.starts_with("addx ") {
    Instruction::AddX(inst[5..].parse().unwrap())
  } else {
    unreachable!()
  }
}

fn main() {
  let test = std::fs::read_to_string("p10/src/input.txt").unwrap();
  let mut processed = Vec::new();
  for instr in test.lines().map(parse_instr) {
    match instr {
      Instruction::Noop => processed.push(instr),
      Instruction::AddX(_) => {
        processed.push(Instruction::Noop);
        processed.push(instr)
      },
    }
  }
  let mut machine = Machine::init(2);
  let mut state = HashMap::new();
  for instr in processed {
    machine.advance(&instr);
    state.insert(machine.cycle, machine.x);
  }
  let cycles = [20, 60, 100, 140, 180, 220];
  let total: i32 = cycles.iter().map(|c| state[&c] * c).sum();
  eprintln!("{}", total);


  machine.render();
}
