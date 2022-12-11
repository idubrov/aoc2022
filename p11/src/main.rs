#[derive(Debug, Clone)]
struct Monkey {
  items: Vec<usize>,
  op: char,
  lhs: Option<usize>,
  rhs: Option<usize>,
  test_div: usize,
  true_monkey: usize,
  false_monkey: usize,
  total: usize,
}

impl Monkey {
  fn from_str(s: &str) -> Monkey {
    let mut it = s.lines();
    assert!(it.next().unwrap().starts_with("Monkey "));
    let starting = it.next().unwrap();
    assert!(starting.starts_with("  Starting items: "));
    let items = starting["  Starting items: ".len()..].split(",").map(|s| s.trim().parse::<usize>().unwrap()).collect::<Vec<_>>();
    let op = it.next().unwrap();
    assert!(op.starts_with("  Operation: new ="));
    let mut ops = op["  Operation: new = ".len()..].split(" ");
    let lhs = ops.next().unwrap();
    let lhs = if lhs == "old" { None } else { Some(lhs.parse::<usize>().unwrap()) };
    let op = ops.next().unwrap().trim().chars().next().unwrap();
    let rhs = ops.next().unwrap();
    let rhs = if rhs == "old" { None } else { Some(rhs.parse::<usize>().unwrap()) };

    let div = it.next().unwrap();
    assert!(div.starts_with("  Test: divisible by "));
    let div = div["  Test: divisible by ".len()..].parse::<usize>().unwrap();

    let t = it.next().unwrap();
    assert!(t.starts_with("    If true: throw to monkey "));
    let t = t["    If true: throw to monkey ".len()..].parse::<usize>().unwrap();
    let f = it.next().unwrap();
    assert!(f.starts_with("    If false: throw to monkey "));
    let f = f["    If false: throw to monkey ".len()..].parse::<usize>().unwrap();
    Monkey {
      items,
      op,
      lhs,
      rhs,
      test_div: div,
      true_monkey: t,
      false_monkey: f,
      total: 0,
    }
  }
}

fn op(monkey: &Monkey, item: usize) -> usize {
  let lhs = monkey.lhs.unwrap_or(item);
  let rhs = monkey.rhs.unwrap_or(item);
  match monkey.op {
    '*' => lhs * rhs,
    '+' => lhs + rhs,
    _ => panic!(),
  }
}

fn monkey(monkeys: &mut [Monkey], monkey: usize, w: bool, worry_div: usize) {
  let items = std::mem::take(&mut monkeys[monkey].items);
  monkeys[monkey].total += items.len();
  for item in items {
    let mut next = op(&monkeys[monkey], item);
    if w {
      next = next / 3;
    }
    let nm = if next % monkeys[monkey].test_div == 0 {
      monkeys[monkey].true_monkey
    } else {
      monkeys[monkey].false_monkey
    };
    //println!("{monkey}: {next} => {nm}");
    next = next % worry_div;
    monkeys[nm].items.push(next);
  }
}

fn round(monkeys: &mut [Monkey], w: bool, worry_div: usize) {
  for idx in 0..monkeys.len() {
    monkey(monkeys, idx, w, worry_div);
  }
}

fn solve(path: &str) {
  let input = std::fs::read_to_string(path).unwrap();
  let mut monkeys = input.split("\n\n")
    .map(|m| Monkey::from_str(m))
    .collect::<Vec<_>>();
  let mut monkeys2 = monkeys.clone();

  let worry_div = monkeys.iter().map(|m| m.test_div).product::<usize>();

  for _ in 0..20 {
    round(&mut monkeys, true, worry_div);
  }
  monkeys.sort_by_key(|m| m.total);
  println!("{} (first): {}", path, monkeys[monkeys.len() - 2].total * monkeys[monkeys.len() - 1].total);

  for _ in 0..10000 {
    round(&mut monkeys2, false, worry_div);
  }
  monkeys2.sort_by_key(|m| m.total);
  println!("{} (second): {}", path, monkeys2[monkeys2.len() - 2].total * monkeys2[monkeys2.len() - 1].total);
}

fn main() {
  solve("p11/src/test.txt");
  solve("p11/src/input.txt");
}
