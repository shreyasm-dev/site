use crate::util::{Metadata, Output};
use std::path::PathBuf;

pub fn sass(input: &[u8], filename: &str) -> Output {
  Output {
    metadata: Metadata::default(),
    content: grass::from_string(
      String::from_utf8(input.to_vec()).unwrap(),
      &grass::Options::default(),
    )
    .unwrap()
    .into_bytes(),
    filename: PathBuf::from(filename)
      .with_extension("css")
      .to_str()
      .unwrap()
      .to_string(),
  }
}
