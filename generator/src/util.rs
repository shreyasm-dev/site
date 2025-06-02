use std::collections::HashMap;

use crate::markdown::render;
use include_dir::Dir;
use proc_macro2::TokenStream;
use quote::quote;
use yaml_rust2::Yaml;

pub fn yaml_to_string(yaml: &Yaml) -> String {
  match yaml {
    Yaml::String(s) => s.to_string(),
    Yaml::Integer(i) => i.to_string(),
    Yaml::Real(r) => r.to_string(),
    Yaml::Boolean(b) => b.to_string(),
    _ => panic!("Unsupported YAML type: {:?}", yaml),
  }
}

pub fn traverse_frontmatter(map: &HashMap<String, HashMap<String, String>>) -> TokenStream {
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
  frontmatter: &mut HashMap<String, HashMap<String, String>>,
) -> TokenStream {
  let mut children = Vec::<TokenStream>::new();

  for entry in dir.entries() {
    if let Some(dir) = entry.as_dir() {
      let tokens = traverse_dir(dir, frontmatter);
      children.push(
        quote! {
          include_dir::DirEntry::Dir(#tokens)
        }
        .into(),
      );
    } else if let Some(file) = entry.as_file() {
      let path = file.path().to_str().unwrap();
      let content = if file.path().extension().is_some_and(|ext| ext == "md") {
        let rendered = render(file.contents_utf8().unwrap());
        frontmatter.insert(path.to_string(), rendered.frontmatter.clone());
        rendered.content.bytes().collect()
      } else {
        file.contents().to_vec()
      };

      children.push(
        quote! {
          include_dir::DirEntry::File(include_dir::File::new(#path, &[#(#content),*]))
        }
        .into(),
      );
    } else {
      unreachable!();
    }
  }

  let path = dir.path().to_str().unwrap();

  quote! {
    include_dir::Dir::new(#path, &[#(#children),*])
  }
}
