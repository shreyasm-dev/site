markup::define! {
  Main<'a>(title: &'a str, content: &'a str) {
    @markup::doctype()
    html {
      head {
        title { "shreyasm" @if !title.is_empty() { " - " @title } }
        meta[charset = "UTF-8"];
        meta[name = "viewport", content = "width=device-width, initial-scale=1.0"];
      }
      body {
        @markup::raw(content)
      }
    }
  }
}
