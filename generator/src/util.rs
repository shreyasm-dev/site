use include_dir::Dir;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use yaml_rust2::Yaml;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
  pub metadata: HashMap<String, String>,
  pub content: String,
}

pub fn yaml_to_string(yaml: &Yaml) -> String {
  match yaml {
    Yaml::String(s) => s.to_string(),
    Yaml::Integer(i) => i.to_string(),
    Yaml::Real(r) => r.to_string(),
    Yaml::Boolean(b) => b.to_string(),
    _ => panic!("unsupported yaml type: {:?}", yaml),
  }
}

pub fn traverse_metadata(map: &HashMap<String, HashMap<String, String>>) -> TokenStream {
  let mut entries = Vec::new();
  for (key, value) in map {
    let key = key.to_string();
    let value = value
      .iter()
      .map(|(k, v)| {
        let k_str = k.to_string();
        let v_str = v.to_string();
        quote! { #k_str => #v_str }
      })
      .collect::<Vec<_>>();

    entries.push(quote! {
      #key => phf::phf_map! {
        #(#value),*
      }
    });
  }

  quote! {
    phf::phf_map! {
      #(#entries),*
    }
  }
}

pub fn traverse_dir<'a>(
  dir: &'a Dir,
  metadata: &mut HashMap<String, HashMap<String, String>>,
  processor: &dyn Fn(&str) -> Output,
) -> TokenStream {
  let mut children = Vec::<TokenStream>::new();

  for entry in dir.entries() {
    if let Some(dir) = entry.as_dir() {
      let path = dir.path().to_str().unwrap();
      let tokens = traverse_dir(dir, metadata, processor);
      children.push(
        quote! {
          #path => #tokens
        }
        .into(),
      );
    } else if let Some(file) = entry.as_file() {
      let path = file.path().to_str().unwrap();
      let content = file.contents_utf8().map(processor);

      if let Some(ref rendered) = content {
        metadata.insert(path.to_string(), rendered.metadata.clone());
      }

      let content = content
        .map(|rendered| rendered.content.bytes().collect())
        .unwrap_or(file.contents().to_vec());

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
