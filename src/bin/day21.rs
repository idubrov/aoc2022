use aoc2022::*;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Parser)]
#[grammar = "bin/day21/formula.pest"]
struct FormulaParser;

#[derive(Debug, Clone, Copy)]
enum Formula<'a> {
  Const(isize),
  Eval { left: &'a str, op: char, right: &'a str },
}

fn parse(s: &str) -> (String, Formula) {
  let mut pairs: Pairs<_> = FormulaParser::parse(Rule::formula, s)
    .unwrap()
    .next()
    .unwrap()
    .into_inner();
  let name = pairs.next_str().to_owned();
  let formula = match pairs.peek().unwrap().as_rule() {
    Rule::val => Formula::Const(pairs.next_parse()),
    _ => {
      let left = pairs.next().unwrap().as_str();
      let op = pairs.next_str().chars().next().unwrap();
      let right = pairs.next().unwrap().as_str();
      Formula::Eval { left, op, right }
    }
  };
  (name, formula)
}

fn eval(map: &HashMap<String, Formula>, key: &str) -> isize {
  match map[key] {
    Formula::Const(result) => result,
    Formula::Eval { left, op, right } => {
      let left = eval(map, left);
      let right = eval(map, right);
      let result = match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => unreachable!(),
      };
      result
    }
  }
}

fn gcd(first: isize, second: isize) -> isize {
  if second == 0 {
    first
  } else {
    gcd(second, first % second)
  }
}

#[derive(Clone, Debug)]
struct Num(isize, isize);
#[derive(Debug)]
struct Val(Num, isize);

impl Mul for Num {
  type Output = Num;

  fn mul(self, rhs: Self) -> Self::Output {
    assert!(self.1 == 0 || rhs.1 == 0);
    Num(self.0 * rhs.0, self.0 * rhs.1 + self.1 * rhs.0)
  }
}

impl Mul<isize> for Num {
  type Output = Num;

  fn mul(mut self, rhs: isize) -> Self::Output {
    self.0 *= rhs;
    self.1 *= rhs;
    self
  }
}

impl Add for Num {
  type Output = Num;

  fn add(mut self, rhs: Self) -> Self::Output {
    self.0 += rhs.0;
    self.1 += rhs.1;
    self
  }
}

impl Sub for Num {
  type Output = Num;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self.0 -= rhs.0;
    self.1 -= rhs.1;
    self
  }
}

impl Add for Val {
  type Output = Val;

  fn add(self, rhs: Self) -> Self::Output {
    Val(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1).normalize()
  }
}

impl Sub for Val {
  type Output = Val;

  fn sub(self, rhs: Self) -> Self::Output {
    Val(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1).normalize()
  }
}

impl Mul for Val {
  type Output = Val;

  fn mul(self, rhs: Self) -> Self::Output {
    Val(self.0 * rhs.0, self.1 * rhs.1).normalize()
  }
}

impl Div for Val {
  type Output = Val;

  fn div(self, rhs: Self) -> Self::Output {
    assert_eq!(rhs.0 .1, 0);
    Val(self.0 * rhs.1, rhs.0 .0 * self.1).normalize()
  }
}

impl Val {
  fn normalize(mut self) -> Self {
    let g = gcd(gcd(self.0 .0, self.0 .1), self.1);
    self.0 .0 /= g;
    self.0 .1 /= g;
    self.1 /= g;
    self
  }
}

fn eval2(map: &HashMap<String, Formula>, key: &str) -> Val {
  match map[key] {
    _ if key == "humn" => Val(Num(0, 1), 1),
    Formula::Const(result) => Val(Num(result, 0), 1),
    Formula::Eval { left, op, right } => {
      let left = eval2(map, left);
      let right = eval2(map, right);
      let result = match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => unreachable!(),
      };
      result
    }
  }
}

fn solve(path: &str) -> (isize, isize) {
  let input = input_data(21, path);
  let mut input = input.lines().map(parse).collect::<HashMap<String, Formula>>();

  let first = eval(&mut input, "root");

  let second = match input["root"] {
    Formula::Eval { left, right, .. } => {
      let left = eval2(&input, left);
      let right = eval2(&input, right);
      let res = left - right;
      -res.0 .0 / res.0 .1
    }
    _ => unreachable!(),
  };

  (first, second)
}

#[test]
fn test() {
  assert_eq!((152, 301), solve("test.txt"));
  assert_eq!((331319379445180, 3715799488132), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
