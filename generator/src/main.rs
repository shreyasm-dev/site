mod image;
mod markdown;
mod minify;
mod sass;
mod util;

use crate::{image::image, markdown::markdown, sass::sass, util::transform};
use std::{collections::HashMap, fs};

fn main() {
  let mut tags = HashMap::new();

  transform("image", image, &mut tags, true);
  transform("page", markdown, &mut tags, true);
  transform("style", sass, &mut tags, true);

  transform("page", markdown, &mut tags, false);

  fs::create_dir_all("out/tags").unwrap();
  for tag in tags.keys() {
    fs::write(
      format!("out/tags/{}.html", tag),
      markdown(format!("---\ntitle = \"{0}\"\n---\n\n# {0}\n\n%{0}", tag).as_bytes(), "", tags.clone())[0]
        .content
        .clone(),
    )
    .unwrap();
  }
}
