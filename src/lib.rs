use std::str::FromStr;
use pest::iterators::Pairs;
use pest::RuleType;
use std::fmt::Debug;

pub fn input_data(day: usize, file: &str) -> String {
  std::fs::read_to_string(format!("src/bin/day{:02}/{}", day, file)).unwrap()
}


pub trait PairsExtra {
  fn next_str(&mut self) -> &str;
  fn next_parse<T: FromStr>(&mut self) -> T where T::Err: Debug;
  fn parse_list<T: FromStr>(&mut self) -> Vec<T> where T::Err: Debug;
}

impl <R: RuleType> PairsExtra for Pairs<'_, R> {
  fn next_str(&mut self) -> &str {
    self.next().unwrap().as_str()
  }

  fn next_parse<T: FromStr>(&mut self) -> T where T::Err: Debug {
    self.next().unwrap().as_str().parse().unwrap()
  }

  fn parse_list<T: FromStr>(&mut self) -> Vec<T> where T::Err: Debug {
    self.next().unwrap().into_inner().map(|item| item.as_str().parse::<T>().unwrap()).collect::<Vec<_>>()
  }
}
