use markdown_it::{
  MarkdownIt, Node, NodeValue, Renderer,
  parser::inline::{InlineRule, InlineState},
};
use markdown_it_front_matter::FrontMatter;
use saphyr::{LoadableYamlNode, Scalar, Yaml};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownOutput {
  frontmatter: HashMap<Yaml<'static>, Yaml<'static>>,
  pub content: String,
}

impl MarkdownOutput {
  pub fn get_frontmatter<'a>(&'a self, key: &'a str) -> Option<&'a Yaml<'static>> {
    self
      .frontmatter
      .get(&Yaml::Value(Scalar::String(key.into())))
  }
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
      inner_html: " ¶".to_string(),
      position: markdown_it_heading_anchors::AnchorPosition::End,
      ..Default::default()
    },
  );
  markdown_it_tasklist::add(md);
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

    if let Ok(fm) = Yaml::load_from_str(&fm.content) {
      if let Some(fm) = fm.first() {
        if let Some(fm) = fm.as_mapping() {
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

    Some((Node::new(Math { content, display }), state.pos - start))
  }
}
