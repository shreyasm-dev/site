use std::{collections::HashMap, fs, path::{Path, PathBuf}};
use toml::{Table, Value};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Output {
  pub content: Vec<u8>,
  pub path: PathBuf,
}

pub fn table_to_map(table: Table) -> HashMap<String, Value> {
  let mut map = HashMap::new();
  for (key, value) in table {
    map.insert(key, value);
  }
  map
}

pub fn transform(kind: &str, transform: fn(&[u8], &str) -> Vec<Output>) {
  std::env::set_current_dir(format!("content/{}", kind)).unwrap();

  for entry in WalkDir::new(".")
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.metadata().unwrap().is_file())
  {
    let data = std::fs::read(entry.path()).unwrap();
    let path = entry.path().to_str().unwrap();
    
    for output in transform(&data, path) {
      let path = Path::new("../../out").join(output.path);

      fs::create_dir_all(path.parent().unwrap()).unwrap();
      fs::write(path, output.content).unwrap();
    }
  }

  std::env::set_current_dir("../..").unwrap();
}
