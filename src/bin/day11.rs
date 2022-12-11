use aoc2022::*;
use std::fmt::Debug;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bin/day11/monkey.pest"]
struct MonkeyParser;

#[derive(Debug, Clone)]
struct Monkey {
  items: Vec<usize>,
  op: String,
  lhs: Option<usize>,
  rhs: Option<usize>,
  test_div: usize,
  true_monkey: usize,
  false_monkey: usize,
  total: usize,
}

fn parse_arg(s: &str) -> Option<usize> {
  if s == "old" {
    None
  } else {
    Some(s.parse::<usize>().unwrap())
  }
}

impl Monkey {
  fn from_str(s: &str) -> Monkey {
    let mut pairs: Pairs<_> = MonkeyParser::parse(Rule::monkey, s).unwrap().next().unwrap().into_inner();
    let m = Monkey {
      items: pairs.parse_list(),
      lhs: parse_arg(pairs.next_str()),
      op: pairs.next_str().to_owned(),
      rhs: parse_arg(pairs.next_str()),
      test_div: pairs.next_parse(),
      true_monkey: pairs.next_parse(),
      false_monkey: pairs.next_parse(),
      total: 0,
    };
    m
  }
}

fn op(monkey: &Monkey, item: usize) -> usize {
  let lhs = monkey.lhs.unwrap_or(item);
  let rhs = monkey.rhs.unwrap_or(item);
  match monkey.op.as_str() {
    "*" => lhs * rhs,
    "+" => lhs + rhs,
    _ => panic!(),
  }
}

fn monkey(monkeys: &mut [Monkey], monkey: usize, worry_drop: bool, worry_div: usize) {
  let items = std::mem::take(&mut monkeys[monkey].items);
  monkeys[monkey].total += items.len();
  for item in items {
    let mut next = op(&monkeys[monkey], item);
    if worry_drop {
      next = next / 3;
    }
    let nm = if next % monkeys[monkey].test_div == 0 {
      monkeys[monkey].true_monkey
    } else {
      monkeys[monkey].false_monkey
    };
    next = next % worry_div;
    monkeys[nm].items.push(next);
  }
}

fn round(monkeys: &mut [Monkey], worry_drop: bool, worry_div: usize) {
  for idx in 0..monkeys.len() {
    monkey(monkeys, idx, worry_drop, worry_div);
  }
}

fn solve(path: &str) {
  let input = input_data(11, path);
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
  solve("test.txt");
  solve("input.txt");
}
