use crate::{
  markdown::markdown,
  minify::{minify_css, minify_html},
  sass::sass,
  util::{Output, table_to_map, traverse_dir, traverse_metadata},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use toml::Value;

static CONTENT: include_dir::Dir<'_> = include_dir::include_dir!("content");

pub fn content(input: TokenStream) -> TokenStream {
  match input.into_iter().collect::<Vec<_>>().as_slice() {
    [] => {}
    _ => panic!("expected no arguments"),
  };

  type F = fn(&[u8], &str) -> Output;
  let (metadata, content, tags) = data(&[
    (
      "page",
      (|input, filename| minify_html(markdown(input, filename))) as F,
    ),
    (
      "style",
      (|input, filename| minify_css(sass(input, filename))) as F,
    ),
  ]);

  quote! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    struct Metadata<'a> {
      pub title: Option<&'a str>,
      pub tags: &'a [&'a str],
    }

    pub struct Content;

    impl Content {
      pub fn of<'a>(
        kind: &'a str,
        path: &'a str,
      ) -> Option<(String, &'a Metadata<'static>)> {
        Self::of_raw(kind, path).map(|(content, metadata)|
          String::from_utf8(content)
            .map(|content| (content, metadata))
            .ok()
        ).flatten()
      }

      pub fn of_raw<'a>(
        kind: &'a str,
        path: &'a str,
      ) -> Option<(Vec<u8>, &'a Metadata<'static>)> {
        static CONTENT: phf::Map<&'static str, phf::Map<&'static str, &[u8]>> = #content;
        static FRONTMATTER: phf::Map<&'static str, Metadata<'static>> = #metadata;
        static EMPTY_METADATA: Metadata<'static> = Metadata {
          title: None,
          tags: &[],
        };

        let path = &format!("{}/{}", kind, path.trim_start_matches("/"));

        CONTENT
          .get(kind)
          .map(|map| map.get(path).map(|content| (content.to_vec(), FRONTMATTER.get(path).unwrap_or(&EMPTY_METADATA))))
          .flatten()
      }

      pub fn tags() -> phf::Map<&'static str, &'static str> {
        #tags
      }
    }
  }
}

pub fn data<F>(input: &[(&str, F)]) -> (TokenStream, TokenStream, TokenStream)
where
  F: Fn(&[u8], &str) -> Output,
{
  let mut metadata = HashMap::new();
  let mut data = Vec::new();

  for (name, processor) in input {
    let content = traverse_dir(&CONTENT.get_dir(name).unwrap(), &mut metadata, &processor);
    data.push((
      name,
      quote! {
        phf::phf_map! {
          #content
        }
      },
    ));
  }

  let data = data.iter().map(|(key, value)| quote! { #key => #value });
  let metadata = traverse_metadata(&metadata);

  let mut tags = Vec::new();
  let tag_table = table_to_map(
    CONTENT
      .get_file("tags.toml")
      .unwrap()
      .contents_utf8()
      .unwrap()
      .parse()
      .unwrap(),
  );

  for (tag, description) in tag_table {
    if let Value::String(description) = description {
      tags.push(quote! {
        #tag => #description
      });
    }
  }

  (
    metadata,
    quote! {
      phf::phf_map! {
        #(#data),*
      }
    },
    quote! {
      phf::phf_map! {
        #(#tags),*
      }
    },
  )
}
