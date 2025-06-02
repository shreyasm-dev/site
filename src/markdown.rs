use markdown_it::{
  MarkdownIt, Node, NodeValue, Renderer,
  parser::inline::{InlineRule, InlineState},
  plugins::html::html_inline::HtmlInline,
};
use markdown_it_front_matter::FrontMatter;
use std::collections::HashMap;
use yaml_rust2::{Yaml, YamlLoader};

use crate::content::components::resource::Resource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownOutput {
  pub frontmatter: HashMap<Yaml, Yaml>,
  pub content: String,
}

pub fn render(markdown: &str) -> MarkdownOutput {
  let md = &mut MarkdownIt::new();
  markdown_it::plugins::cmark::add(md);
  markdown_it::plugins::extra::add(md);
  markdown_it::plugins::html::add(md);
  markdown_it::plugins::sourcepos::add(md);
  markdown_it_autolink::add(md);
  markdown_it_footnote::add(md);
  markdown_it_front_matter::add(md);
  markdown_it_heading_anchors::add_with_options(
    md,
    markdown_it_heading_anchors::HeadingAnchorOptions {
      id_on_heading: true,
      inner_html: "¶".to_string(),
      position: markdown_it_heading_anchors::AnchorPosition::End,
      ..Default::default()
    },
  );
  markdown_it_tasklist::add(md);

  md.inline.add_rule::<ComponentScanner>();
  md.inline.add_rule::<MathScanner>();

  let mut output = md.parse(markdown);
  let mut frontmatter = HashMap::new();

  if output
    .children
    .first()
    .map(|node| node.node_type.name == "markdown_it_front_matter::FrontMatter")
    .unwrap_or(false)
  {
    let fm = output.children.remove(0);
    let fm: &FrontMatter = fm.cast().unwrap(); // should be safe

    if let Ok(fm) = YamlLoader::load_from_str(&fm.content) {
      if let Some(fm) = fm.first() {
        if let Some(fm) = fm.as_hash() {
          for (key, value) in fm {
            frontmatter.insert(key.clone(), value.clone());
          }
        }
      }
    }
  }

  MarkdownOutput {
    frontmatter,
    content: md.parse(markdown).render(),
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Math {
  pub content: String,
  pub display: bool,
}

impl NodeValue for Math {
  fn render(&self, _node: &Node, fmt: &mut dyn Renderer) {
    let mathml = katex::render_with_opts(
      &self.content,
      katex::Opts::builder()
        .output_type(katex::OutputType::Mathml)
        .display_mode(self.display)
        .build()
        .unwrap_or_default(), // safe, but we go with the default just in case
    )
    .unwrap_or("<span class=\"katex-error\">KaTeX rendering failed</span>".to_string());

    fmt.text_raw(&mathml);
  }
}

struct MathScanner;

impl InlineRule for MathScanner {
  const MARKER: char = '$';

  fn run(state: &mut InlineState) -> Option<(Node, usize)> {
    let start = state.pos;
    let display;
    let mut content = String::new();

    display = state.src[state.pos..].starts_with("$$");
    state.pos += 1 + (display as usize);

    while state.pos < state.src.len() {
      if state.src[state.pos..].starts_with(if display { "$$" } else { "$" }) {
        state.pos += 1 + (display as usize);
        break;
      }

      content.push(state.src[state.pos..].chars().next().unwrap_or_default());
      state.pos += 1;
    }

    let len = state.pos - start;
    state.pos = start;
    Some((Node::new(Math { content, display }), len))
  }
}

struct ComponentScanner;

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentArg {
  String(String),
  Bool(bool),
  Number(f64),
}

impl InlineRule for ComponentScanner {
  const MARKER: char = '@';

  fn run(state: &mut InlineState) -> Option<(Node, usize)> {
    let start = state.pos;
    let mut args = Vec::new();
    let mut name = String::new();

    state.pos += 1;

    while let Some(c) = state.src[state.pos..].chars().next() {
      if !c.is_alphanumeric() && c != '_' && c != '-' {
        break;
      }

      name.push(c);
      state.pos += 1;
    }

    if state.src[state.pos..].starts_with('(') {
      state.pos += 1;

      let mut current_arg = String::new();
      let mut in_quotes = false;
      let mut escape_next = false;

      while let Some(c) = state.src[state.pos..].chars().next() {
        if escape_next {
          current_arg.push(c);
          escape_next = false;
        } else if c == '\\' {
          escape_next = true;
        } else if c == '"' {
          in_quotes = !in_quotes;
          current_arg.push(c);
        } else if c == ',' && !in_quotes {
          args.push(current_arg.trim().to_string());
          current_arg.clear();
        } else if c == ')' && !in_quotes {
          if !current_arg.is_empty() {
            args.push(current_arg.trim().to_string());
            current_arg.clear();
          }
          state.pos += 1;
          break;
        } else {
          current_arg.push(c);
        }

        state.pos += 1;
      }
    } else {
      args.push(name.clone());
      name.clear();
    }

    let args: Vec<ComponentArg> = args
      .into_iter()
      .map(|arg| {
        if let Ok(num) = arg.parse::<f64>() {
          ComponentArg::Number(num)
        } else if arg == "true" || arg == "false" {
          ComponentArg::Bool(arg == "true")
        } else {
          ComponentArg::String(arg)
        }
      })
      .collect();

    let len = state.pos - start;
    state.pos = start;
    match name.as_str() {
      "resource" => Some(
        Resource {
          kind: &args
            .get(0)
            .and_then(|arg| match arg {
              ComponentArg::String(s) => Some(s.clone()),
              _ => None,
            })
            .unwrap_or_default(),
          path: &args
            .get(1)
            .and_then(|arg| match arg {
              ComponentArg::String(s) => Some(s.clone()),
              _ => None,
            })
            .unwrap_or_default(),
        }
        .to_string(),
      ),
      "hello" => Some(format!(
        "hello, {}!",
        args
          .get(0)
          .and_then(|arg| match arg {
            ComponentArg::String(s) => Some(s.clone()),
            _ => None,
          })
          .unwrap_or("world".to_string())
      )),
      _ => None,
    }
    .map(|content| (Node::new(HtmlInline { content }), len))
  }
}
