fn main() {
  for file in std::fs::read_dir(".").unwrap() {
    let file = file.unwrap();
    let name = file.file_name();
    let name = name.to_str().unwrap();
    if name.starts_with("p") {
      let new_name = "day".to_string() + &name[1..];
      std::fs::create_dir_all(format!("src/bin/{new_name}")).unwrap();
      std::fs::copy(format!("{name}/src/main.rs"), format!("src/bin/{new_name}.rs")).unwrap();
      for data_file in std::fs::read_dir(format!("{name}/src")).unwrap() {
        let data_file = data_file.unwrap();
        let data_name = data_file.file_name();
        let data_name = data_name.to_str().unwrap();
        if data_name.ends_with(".txt") {
          std::fs::copy(
            format!("{name}/src/{data_name}"),
            format!("src/bin/{new_name}/{data_name}"),
          )
          .unwrap();
        }
      }
    }
  }
}
