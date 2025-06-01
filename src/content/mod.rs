pub mod components;
pub mod main;

use include_directory::Dir;
use std::path::Path;

static CONTENT: Dir<'_> = include_directory::include_directory!("content");

pub struct Content;

impl Content {
  pub fn get() -> &'static Dir<'static> {
    &CONTENT
  }

  pub fn of(kind: &str, path: &str) -> Option<String> {
    Self::get()
      .get_file(Path::new(kind).join(path))
      .and_then(|file| file.contents_utf8())
      .map(|s| s.to_string())
  }
}
