use super::style::Style;

markup::define! {
  Main<'a>(title: Option<&'a str>, content: &'a str) {
    @markup::doctype()
    html[lang = "en"] {
      head {
        title { "-" @if let Some(title) = title { " - " @title } }
        meta[charset = "UTF-8"];
        meta[name = "viewport", content = "width=device-width, initial-scale=1.0"];
        @Style { name: "normalize.min" }
        @Style { name: "main" }
      }
      body {
        header {

        }

        main {
          @markup::raw(content)
        }

        footer {

        }
      }
    }
  }
}
