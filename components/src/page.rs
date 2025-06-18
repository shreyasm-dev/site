use super::style::Style;

markup::define! {
  Main<'a>(title: Option<&'a str>, content: &'a str) {
    @markup::doctype()
    html[lang = "en"] {
      head {
        title { "Shreyas M" @if let Some(title) = title { " - " @title } }
        meta[charset = "UTF-8"];
        meta[name = "viewport", content = "width=device-width, initial-scale=1.0"];
        @Style { name: "normalize" }
        @Style { name: "main" }
      }
      body {
        header {
          h2 {
            a[href = "/"] {
              "Shreyas M"
            }
          }
        }

        main {
          @markup::raw(content)
        }

        footer {
          "All first-party content on this website is, unless explicitly mentioned otherwise, licensed under " a[href = "https://creativecommons.org/licenses/by-nc-sa/4.0/"] { "CC BY-NC-SA 4.0" }
        }
      }
    }
  }
}
