use crate::Content;

markup::define! {
  Style<'a>(path: &'a str) {
    style {
      @if let Some(data) = Content::of("style", format!("{}.css", path).as_str()) {
        @markup::raw(data)
      } else {
        "/* style " @path  " not found */"
      }
    }
  }
}
