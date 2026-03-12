use super::style::Style;
use crate::types::Metadata;

markup::define! {
  Main<'a>(metadata: Metadata, content: &'a str) {
    @markup::doctype()
    html[lang = "en"] {
      head {
        title { @if let Some(title) = &metadata.title { @title } else { "Shreyas" } }
        meta[charset = "UTF-8"];
        meta[name = "viewport", content = "width=device-width, initial-scale=1.0"];
        @Style { name: "normalize" }
        @Style { name: "main" }
      }
      body {
        header {
          h2 {
            a[href = "/"] {
              "Shreyas"
            }
          }
        }

        main {
          @if let Some(date) = metadata.date {
            p .caption {
              @date.format("%e %B, %Y").to_string()
            }
          }

          @if metadata.tags.len() > 0 {
            p .tags {
              @for tag in metadata.tags.clone() {
                a .tag [href = format!("/tags/{}", tag)] {
                  @tag
                }
              }
            }
          }

          @if metadata.date.is_some() || metadata.tags.len() > 0 {
            hr;
          }

          @markup::raw(content)

          hr;
        }

        footer {
          "All first-party content on this website and " a[href = "https://github.com/shreyasm-dev/site"] { "the code used to generated it" } " are, to whatever degree possible and unless explicitly mentioned otherwise, licensed under " a[href = "https://creativecommons.org/licenses/by-nc-sa/4.0/"] { "CC BY-NC-SA 4.0" }
        }
      }
    }
  }
}
