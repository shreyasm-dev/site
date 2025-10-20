use crate::{minify::minify_css, util::Output};
use std::path::PathBuf;

pub fn sass(input: &[u8], path: &str) -> Vec<Output> {
  vec![minify_css(Output {
    // metadata: Metadata::default(),
    content: grass::from_string(
      String::from_utf8(input.to_vec()).unwrap(),
      &grass::Options::default(),
    )
    .unwrap()
    .into_bytes(),
    path: PathBuf::from(path).with_extension("css"),
  })]
}
