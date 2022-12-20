use std::cell::RefCell;
use std::rc::Rc;
use aoc2022::*;

struct Node {
  value: isize,
  prev: RefCell<Option<Rc<Node>>>,
  next: RefCell<Option<Rc<Node>>>,
}

fn remove(node: Rc<Node>) {
  let next = node.next.borrow().clone().unwrap();
  let prev = node.prev.borrow().clone().unwrap();
  *prev.next.borrow_mut() = Some(next.clone());
  *next.prev.borrow_mut() = Some(prev.clone());

  *node.prev.borrow_mut() = None;
  *node.next.borrow_mut() = None;
}

fn insert(prev: Rc<Node>, node: Rc<Node>) {
  let next = prev.next.borrow().clone().unwrap();

  *prev.next.borrow_mut() = Some(node.clone());
  *next.prev.borrow_mut() = Some(node.clone());

  *node.next.borrow_mut() = Some(next);
  *node.prev.borrow_mut() = Some(prev);
}

fn shift(item: Rc<Node>, mul: isize, len: usize) {
  let wrap = (len - 1) as isize;
  let mut count = (item.value * mul) % wrap;
  if count < 0 {
    count += wrap;
  }
  let prev = item.prev.borrow().clone().unwrap();
  remove(item.clone());
  let it = skip(prev, count);
  insert(it, item);
}

fn skip(mut it: Rc<Node>, count: isize) -> Rc<Node> {
  assert!(count >= 0);
  for _ in 0..count {
    let next = it.next.borrow().clone().unwrap();
    it = next;
  }
  it
}

fn solve_inner(path: &str, mul: isize, iter: usize) -> isize {
  let first_list = input_data(20, path)
    .lines()
    .map(|line| line.parse::<isize>().unwrap())
    .map(|value| Rc::new(Node {
      value,
      prev: RefCell::new(None),
      next: RefCell::new(None),
    }))
    .collect::<Vec<_>>();

  for idx in 0..first_list.len() {
    let next = (idx + 1) % first_list.len();
    let prev = (idx + first_list.len() - 1) % first_list.len();
    *first_list[idx].prev.borrow_mut() = Some(first_list[prev].clone());
    *first_list[idx].next.borrow_mut() = Some(first_list[next].clone());
  }

  for _ in 0..iter {
    for idx in 0..first_list.len() {
      shift(first_list[idx].clone(), mul, first_list.len());
    }
  }
  let zero = first_list.iter().find(|n| n.value == 0).unwrap();
  let sum = skip(zero.clone(), 1000).value + skip(zero.clone(), 2000).value + skip(zero.clone(), 3000).value;
  sum * mul
}

fn solve(path: &str) -> (isize, isize) {
  let first = solve_inner(path, 1, 1);
  let second = solve_inner(path, 811589153, 10);
  (first, second)
}

#[test]
fn test() {
  assert_eq!((3, 1623178306), solve("test.txt"));
  assert_eq!((8764, 535648840980), solve("input.txt"));
}

fn main() {
  let test = solve("test.txt");
  println!("test.txt: {} and {}", test.0, test.1);

  let input = solve("input.txt");
  println!("input.txt: {} and {}", input.0, input.1);
}
