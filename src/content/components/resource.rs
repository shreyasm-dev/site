use crate::Content;

markup::define! {
  Resource<'a>(kind: &'a str, path: &'a str) {
    @if let Some(data) = Content::of(kind, path) {
      @markup::raw(format!("<{}>{}</{}>", kind, data, kind))
    }
  }
}
