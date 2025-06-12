use crate::util::Output;
use minify_html_onepass::{Cfg, copy};

pub fn minify_html(input: Output) -> Output {
  Output {
    content: copy(
      &input.content,
      &Cfg {
        minify_css: true,
        minify_js: true,
      },
    )
    .unwrap(),
    ..input
  }
}
