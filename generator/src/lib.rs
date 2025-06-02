mod markdown;
mod util;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use util::{traverse_dir, traverse_frontmatter};

static CONTENT: include_dir::Dir<'_> = include_dir::include_dir!("content");

#[proc_macro]
pub fn content(input: TokenStream) -> TokenStream {
  match input.into_iter().collect::<Vec<_>>().as_slice() {
    [] => {}
    _ => panic!("expected no arguments"),
  };

  let mut frontmatter = HashMap::new();
  let data = traverse_dir(&CONTENT, &mut frontmatter);
  let frontmatter = traverse_frontmatter(&frontmatter);

  quote! {
    pub struct Content;

    impl Content {
      pub fn of<'a>(
        kind: &'a str,
        path: &'a str,
      ) -> Option<(String, Option<&'a phf::Map<&'static str, &'static str>>)> {

        static CONTENT: include_dir::Dir<'_> = #data;
        static FRONTMATTER: phf::Map<&'static str, phf::Map<&'static str, &'static str>> = #frontmatter;

        let path = std::path::Path::new(kind).join(path.trim_start_matches("/"));
        let path_str = path.clone();
        let path_str = path_str.to_str().unwrap();

        CONTENT.get_file(path).and_then(|file| {
          file
            .contents_utf8()
            .map(|s| (s.to_string(), FRONTMATTER.get(path_str)))
        })
      }
    }
  }
  .into()
}
