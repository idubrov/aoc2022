use aoc2022::*;
use std::collections::HashMap;

fn main() {
  let mut files: HashMap<String, i32> = HashMap::new();
  let mut prefix = "/".to_owned();
  files.insert("/".to_owned(), 0);

  let input = input_data(7, "input.txt");
  let mut it = input.lines().peekable();
  while let Some(command) = it.next() {
    match command {
      "$ cd .." => {
        let pos = prefix[0..prefix.len() - 1].rfind("/").unwrap();
        prefix = prefix[..pos + 1].to_owned();
      }
      "$ cd /" => {
        prefix = "/".to_owned();
      }
      _ if command.starts_with("$ cd ") => {
        prefix += &command[5..];
        prefix += "/"
      }
      "$ ls" => {
        while it.peek().map_or(false, |s| !s.starts_with("$")) {
          let entry = it.next().unwrap();
          if entry.starts_with("dir ") {
            let name = prefix.to_owned() + &entry[4..] + "/";
            files.insert(name, 0);
          } else {
            let mut it2 = entry.splitn(2, " ");
            let size = it2.next().unwrap().parse::<i32>().unwrap();
            let name = prefix.to_owned() + it2.next().unwrap();
            files.insert(name, size);
          }
        }
      }
      _ => unreachable!("invalid '{}'", command),
    }
  }
  let mut sizes = HashMap::new();
  for dir in files.keys() {
    if !dir.ends_with("/") {
      continue;
    }

    let mut total = 0;
    for (file, size) in &files {
      if file.starts_with(dir) {
        total += size;
      }
    }
    sizes.insert(dir, total);
  }
  let mut tot = 0;
  for (name, size) in &sizes {
    if *name != "/" && *size <= 100000 {
      tot += size;
    }
  }
  let mut min_size = sizes[&"/".to_owned()];
  let mut min_name = "/".to_owned();
  let need = 30000000 - (70000000 - sizes[&"/".to_owned()]);
  eprintln!("{:?}", sizes);
  eprintln!("{}", tot);
  for (name, size) in &sizes {
    if *size >= need && *size < min_size {
      min_size = *size;
      min_name = name.to_string();
    }
  }
  eprintln!("{}", min_size);
}
