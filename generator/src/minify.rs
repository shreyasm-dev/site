use crate::util::Output;

pub fn minify_html(input: Output) -> Output {
  Output {
    content: minify_html_onepass::copy(
      &input.content,
      &minify_html_onepass::Cfg {
        minify_css: true,
        minify_js: true,
      },
    )
    .unwrap(),
    ..input
  }
}

pub fn minify_css(input: Output) -> Output {
  Output {
    content: css_minify::optimizations::Minifier::default()
      .minify(
        &String::from_utf8(input.content).unwrap(),
        css_minify::optimizations::Level::Three,
      )
      .unwrap()
      .into_bytes(),
    ..input
  }
}
