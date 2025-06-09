use crate::{
  markdown::markdown,
  util::{traverse_dir, traverse_metadata},
  verbatim::verbatim,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

static CONTENT: include_dir::Dir<'_> = include_dir::include_dir!("content");

pub fn content(input: TokenStream) -> TokenStream {
  match input.into_iter().collect::<Vec<_>>().as_slice() {
    [] => {}
    _ => panic!("expected no arguments"),
  };

  let mut metadata = HashMap::new();

  let page = traverse_dir(&CONTENT.get_dir("page").unwrap(), &mut metadata, &markdown);
  let style = traverse_dir(&CONTENT.get_dir("style").unwrap(), &mut metadata, &verbatim);

  let content = [
    (
      "page",
      quote! {
        phf::phf_map! {
          #page
        }
      },
    ),
    (
      "style",
      quote! {
        phf::phf_map! {
          #style
        }
      },
    ),
  ]
  .map(|(key, value)| quote! { #key => #value });

  let metadata = traverse_metadata(&metadata);
  let content = quote! {
    phf::phf_map! {
      #(#content),*
    }
  };

  quote! {
    pub struct Content;

    impl Content {
      pub fn of<'a>(
        kind: &'a str,
        path: &'a str,
      ) -> Option<(String, &'a phf::Map<&'static str, &'static str>)> {
        Self::of_raw(kind, path).map(|(content, metadata)|
          String::from_utf8(content)
            .map(|content| (content, metadata))
            .ok()
        ).flatten()
      }

      pub fn of_raw<'a>(
        kind: &'a str,
        path: &'a str,
      ) -> Option<(Vec<u8>, &'a phf::Map<&'static str, &'static str>)> {
        static CONTENT: phf::Map<&'static str, phf::Map<&'static str, &[u8]>> = #content;
        static FRONTMATTER: phf::Map<&'static str, phf::Map<&'static str, &'static str>> = #metadata;
        static EMPTY: phf::Map<&'static str, &'static str> = phf::phf_map! {};

        let path = &format!("{}/{}", kind, path.trim_start_matches("/"));

        CONTENT
          .get(kind)
          .map(|map| map.get(path).map(|content| (content.to_vec(), FRONTMATTER.get(path).unwrap_or(&EMPTY))))
          .flatten()
      }
    }
  }
}
