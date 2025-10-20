mod image;
mod markdown;
mod minify;
mod sass;
mod util;

use crate::{image::image, markdown::markdown, sass::sass, util::transform};

fn main() {
  // page => md, minify
  // style => sass, minify
  // image => exif

  transform("image", image);
  transform("page", markdown);
  transform("style", sass);
}
