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
    (#data, #frontmatter)
  }
  .into()
}
