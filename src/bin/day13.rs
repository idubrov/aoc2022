use std::cmp::Ordering;
use aoc2022::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
  List(Vec<Node>),
  Value(usize),
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Node::Value(l), Node::Value(r)) => l.cmp(r),
      (Node::List(l), Node::List(r)) => {
        for idx in 0.. {
          if idx < l.len() && idx < r.len() {
            if l[idx] < r[idx] {
              return Ordering::Less;
            } else if l[idx] > r[idx] {
              return Ordering::Greater;
            }
          } else if l.len() < r.len() {
            return Ordering::Less;
          } else if l.len() > r.len() {
            return Ordering::Greater;
          } else {
            break;
          }
        }
        return Ordering::Equal;
      },
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

struct Parser<'a> {
  text: &'a [u8],
  pos: usize,
}

impl Parser<'_> {
  fn next(&self) -> u8 {
    self.text[self.pos]
  }
  fn parse_text(text: &str) -> Node {
    let mut parser = Parser {
      text: text.as_bytes(),
      pos: 0
    };
    parser.parse()
  }
  fn parse(&mut self) -> Node {
    if self.next() == b'[' {
      self.pos += 1;
      let mut list = Vec::new();
      if self.next() == b']' {
        self.pos += 1;
        return Node::List(list);
      }
      list.push(self.parse());
      while self.next() == b',' {
        self.pos += 1;
        list.push(self.parse());
      }
      assert_eq!(self.next(), b']');
      self.pos += 1;
      return Node::List(list);
    } else {
      let len = self.text[self.pos..].iter().position(|ch| *ch < b'0' || *ch > b'9').unwrap_or(self.text[self.pos..].len());
      let value = String::from_utf8_lossy(&self.text[self.pos..self.pos + len]).parse::<usize>().unwrap();
      self.pos += len;
      return Node::Value(value);
    }
  }
}


fn solve(path: &str) {
  let data = input_data(13, path);
  let data = data.split("\n\n").collect::<Vec<_>>();

  let mut total = 0;
  let mut list = Vec::new();
  for (idx, entry) in data.iter().enumerate() {
    let (first, second) = entry.split_once("\n").unwrap();

    let first = Parser::parse_text(first);
    let second = Parser::parse_text(second);
    if first < second {
      total += idx + 1;
    }
    list.push(first);
    list.push(second);
  }

  let div1 = Parser::parse_text("[[2]]");
  let div2 = Parser::parse_text("[[6]]");

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
