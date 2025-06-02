use super::components::resource::Resource;

markup::define! {
  Main<'a>(title: Option<&'a str>, content: &'a str) {
    @markup::doctype()
    html[lang = "en"] {
      head {
        title { "shreyasm" @if let Some(title) = title { " - " @title } }
        meta[charset = "UTF-8"];
        meta[name = "viewport", content = "width=device-width, initial-scale=1.0"];
        @Resource { kind: "style", path: "normalize.min.css" }
        @Resource { kind: "style", path: "main.css" }
      }
      body {
        @markup::raw(content)
      }
    }
  }
}
