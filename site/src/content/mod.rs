pub mod components;
pub mod main;

use include_dir::Dir;
use phf::Map;
use std::path::Path;

static CONTENT: (Dir<'_>, Map<&'static str, Map<&'static str, &'static str>>) =
  generator::content!();

pub struct Content;

impl Content {
  pub fn of<'a>(
    kind: &'a str,
    path: &'a str,
  ) -> Option<(String, Option<&'a Map<&'static str, &'static str>>)> {
    let path = Path::new(kind).join(path.trim_start_matches("/"));
    let path_str = path.clone();
    let path_str = path_str.to_str().unwrap();

    CONTENT.0.get_file(path).and_then(|file| {
      file
        .contents_utf8()
        .map(|s| (s.to_string(), CONTENT.1.get(path_str)))
    })
  }
}
