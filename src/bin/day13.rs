use std::cmp::Ordering;
use pest::iterators::Pair;
use pest_derive::Parser;
use pest::Parser;
use aoc2022::*;

#[derive(Parser)]
#[grammar = "bin/day13/list.pest"]
struct ListParser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
  List(Vec<Node>),
  Value(usize),
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Node::Value(l), Node::Value(r)) => l.cmp(r),
      (Node::List(l), Node::List(r)) => l.cmp(r),
      (lhs @ Node::Value(_), rhs @ Node::List(_)) => Node::List(vec![lhs.to_owned()]).cmp(rhs),
      (lhs @ Node::List(_), rhs @ Node::Value(_)) => lhs.cmp(&Node::List(vec![rhs.to_owned()])),
    }
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn parse_pair(pair: Pair<Rule>) -> Node {
  match pair.as_rule() {
    Rule::list => Node::List(pair.into_inner().map(parse_pair).collect::<Vec<_>>()),
    Rule::value => Node::Value(pair.as_str().parse().unwrap()),
    _ => unreachable!()
  }
}

fn parse_list(text: &str) -> Node {
  parse_pair(ListParser::parse(Rule::node, text).unwrap().next().unwrap())
}

fn solve(path: &str) {
  let data = input_data(13, path);
  let data = data.split("\n\n").collect::<Vec<_>>();
  let pairs = data
    .iter()
    .map(|item| item.split_once("\n").unwrap())
    .map(|(f, s)| (parse_list(f), parse_list(s)))
    .collect::<Vec<_>>();

  let total = pairs.iter()
    .enumerate()
    .filter(|(_, (a, b))| a < b).map(|(idx, _)| idx + 1)
    .sum::<usize>();

  let mut list = pairs.into_iter().flat_map(|(f, s)| [f, s].into_iter()).collect::<Vec<_>>();
  let div1 = parse_list("[[2]]");
  let div2 = parse_list("[[6]]");
  list.push(div1.clone());
  list.push(div2.clone());
  list.sort();

  let div1_idx = list.iter().position(|n| n == &div1).unwrap() + 1;
  let div2_idx = list.iter().position(|n| n == &div2).unwrap() + 1;

  println!("{} (first): {}", path, total);
  println!("{} (second): {}", path, div1_idx * div2_idx);
}

fn main() {
  solve("test.txt");
  solve("input.txt");
}
