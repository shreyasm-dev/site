use include_dir::Dir;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::collections::HashMap;
use toml::{Table, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
  pub metadata: Metadata,
  pub content: Vec<u8>,
  pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
  pub title: Option<String>,
  pub tags: Vec<String>,
  pub exif: Option<Exif>,
}

impl Default for Metadata {
  fn default() -> Self {
    Self {
      title: None,
      tags: Vec::new(),
      exif: None,
    }
  }
}

impl Into<TokenStream> for Metadata {
  fn into(self) -> TokenStream {
    let title = if let Some(title) = self.title {
      quote! { Some(#title) }
    } else {
      quote! { None }
    };
    let tags = self.tags;
    let exif = self.exif;

    quote! {
      Metadata {
        title: #title,
        tags: &[#(#tags),*],
        exif: #exif,
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exif {
  pub make: Option<String>,
  pub model: Option<String>,
  pub lens: Option<String>,
  pub aperture: Option<(u32, u32)>,
  pub f: Option<(u32, u32)>,
  pub iso: Option<u16>,
  pub iso_speed: Option<u32>,
  pub exposure_time: Option<(u32, u32)>,
  pub focal_length: Option<(u32, u32)>,
}

impl Into<TokenStream> for Exif {
  fn into(self) -> TokenStream {
    quote! {
      Exif {}
    }
  }
}

impl ToTokens for Exif {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend(Into::<TokenStream>::into(self.clone()));
  }
}

pub fn traverse_metadata(map: &HashMap<String, Metadata>) -> TokenStream {
  let mut entries = Vec::new();
  for (key, value) in map {
    let key = key.to_string();
    let value = Into::<TokenStream>::into(value.clone());

    entries.push(quote! {
      #key => #value
    });
  }

  quote! {
    phf::phf_map! {
      #(#entries),*
    }
  }
}

pub fn table_to_map(table: Table) -> HashMap<String, Value> {
  let mut map = HashMap::new();
  for (key, value) in table {
    map.insert(key, value);
  }
  map
}

pub fn traverse_dir<'a>(
  dir: &'a Dir,
  metadata: &mut HashMap<String, Metadata>,
  processor: &dyn Fn(&[u8], &str) -> Output,
) -> TokenStream {
  let mut children = Vec::new();

  for entry in dir.entries() {
    if let Some(dir) = entry.as_dir() {
      children.push(traverse_dir(dir, metadata, processor));
    } else if let Some(file) = entry.as_file() {
      let path = file.path().to_str().unwrap();
      let content = processor(
        file.contents(),
        file.path().file_name().unwrap().to_str().unwrap(),
      );

      metadata.insert(path.to_string(), content.metadata.clone());

      let path = file.path().with_file_name(content.filename);
      let path = path.to_str().unwrap();
      let content = content.content;

      children.push(
        quote! {
          #path => &[#(#content),*]
        }
        .into(),
      );
    } else {
      unreachable!();
    }
  }

  quote! {
    #(#children),*
  }
}
