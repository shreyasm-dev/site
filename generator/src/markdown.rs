use crate::{
  minify::minify_html,
  util::{Output, table_to_map},
};
use chrono::NaiveDate;
use components::{page::Main, style::Style, types::Metadata};
use itertools::Itertools;
use lazy_regex::regex_replace_all;
use markdown_it::{
  MarkdownIt, Node, NodeValue, Renderer,
  parser::inline::{InlineRule, InlineState, Text},
  plugins::html::html_inline::HtmlInline,
};
use markdown_it_front_matter::FrontMatter;
use std::{collections::HashMap, convert::identity, path::PathBuf};
use toml::{Table, Value};

pub fn render_markdown(content: String) -> (HashMap<std::string::String, Value>, String) {
  let md = &mut MarkdownIt::new();
  markdown_it::plugins::cmark::add(md);
  markdown_it::plugins::extra::add(md);
  markdown_it::plugins::html::add(md);
  // markdown_it::plugins::sourcepos::add(md);
  markdown_it_autolink::add(md);
  markdown_it_footnote::add(md);
  markdown_it_front_matter::add(md);
  markdown_it_heading_anchors::add_with_options(
    md,
    markdown_it_heading_anchors::HeadingAnchorOptions {
      id_on_heading: true,
      inner_html: "#".to_string(),
      position: markdown_it_heading_anchors::AnchorPosition::Start,
      ..Default::default()
    },
  );
  markdown_it_tasklist::add(md);

  md.inline.add_rule::<MathScanner>();
  md.inline.add_rule::<ComponentScanner>();
  md.inline.add_rule::<EmojiScanner>();

  let mut output = md.parse(&content);
  let mut frontmatter = HashMap::new();

  if output
    .children
    .first()
    .map(|node| node.node_type.name == "markdown_it_front_matter::FrontMatter")
    .unwrap_or(false)
  {
    let fm = output.children.remove(0);
    let fm: &FrontMatter = fm.cast().unwrap(); // should be safe

    if let Ok(table) = fm.content.parse::<Table>().map(table_to_map) {
      frontmatter = table;
    }
  }

  (frontmatter, output.render())
}

pub fn markdown(markdown: &[u8], path: &str, tags: HashMap<String, Vec<Output>>) -> Vec<Output> {
  let markdown = String::from_utf8(markdown.to_vec()).unwrap();
  let (frontmatter, output) = render_markdown(markdown);

  let metadata = Metadata {
    title: frontmatter
      .get("title")
      .and_then(|d| d.as_str().map(|s| s.to_string())),
    date: frontmatter.get("date").and_then(|d| {
      d.as_str()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
    }),
    tags: frontmatter
      .get("tags")
      .map(|d| {
        d.as_array().map(|a| {
          a.iter()
            .map(|d| d.as_str().map(|s| s.to_string()))
            .filter_map(identity)
            .collect()
        })
      })
      .flatten()
      .unwrap_or_default(),
  };

  vec![minify_html(Output {
    metadata: metadata.clone(),
    content: Main {
      metadata,
      content: &regex_replace_all!("<p>%([a-z-]+)</p>", &output, |_, tag| {
        // TODO: very janky
        let items = tags
          .get(tag)
          .cloned()
          .unwrap_or(Vec::new())
          .into_iter()
          .sorted_by_key(|k| k.metadata.date)
          .rev()
          .chunk_by(|k| k.metadata.date.map(|d| d.format("%Y").to_string()));
        let items = items.into_iter().map(|v| {
          (
            match v.0 {
              None => "Undated".to_string(),
              Some(y) => y,
            },
            v.1.sorted_by_key(|k| k.metadata.date).rev(),
          )
        });

        render_markdown(
          items
            .map(|(date, posts)| {
              format!(
                "## {}\n\n{}",
                date,
                posts
                  .map(|output| format!(
                    "<div style=\"display: flex; flex-direction: row;\"><a href=\"/{}\" style=\"flex-grow: 1;\">{}</a><span>{}</span></div>",
                    output.path.with_extension("").display(),
                    output.metadata.title.unwrap_or("No Title".to_string()),
                    output.metadata.date.map(|date| date.format("%e %B, %Y").to_string()).unwrap_or_default(),
                  ))
                  .join("<br />")
              )
            })
            .join("\n\n"),
        )
        .1
      }),
    }
    .to_string()
    .into_bytes(),
    path: PathBuf::from(path).with_extension("html"),
  })]
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
    if !state.src[state.pos..].starts_with(Self::MARKER) {
      return None;
    }

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
    if !state.src[state.pos..].starts_with(Self::MARKER) {
      return None;
    }

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
      "style" => {
        if args.len() != 1 {
          return None; // style component requires exactly one argument
        }

        match args.get(0) {
          Some(ComponentArg::String(s)) => Some(s),
          _ => None,
        }
        .map(|name| Style { name }.to_string())
        .map(|content| (Node::new(HtmlInline { content }), len))
      }
      "hello" => Some((
        Node::new(Text {
          content: format!(
            "hello, {}!",
            args
              .get(0)
              .and_then(|arg| match arg {
                ComponentArg::String(s) => Some(s.clone()),
                _ => None,
              })
              .unwrap_or("world".to_string())
          ),
        }),
        len,
      )),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Emoji {
  pub name: String,
  pub offset: usize,
}

impl NodeValue for Emoji {
  fn render(&self, _node: &Node, fmt: &mut dyn Renderer) {
    let result = emoji::search::search_name(&self.name);
    fmt.text(
      result
        .get(self.offset)
        .or(result.first())
        .map(|e| e.glyph)
        .unwrap_or_default(),
    );
  }
}

struct EmojiScanner;

impl InlineRule for EmojiScanner {
  const MARKER: char = '{';

  fn run(state: &mut InlineState) -> Option<(Node, usize)> {
    if !state.src[state.pos..].starts_with(Self::MARKER) {
      return None;
    }

    let start = state.pos;
    let mut name = String::new();
    let mut offset = 0;

    state.pos += 1;

    while let Some(c) = state.src[state.pos..].chars().next() {
      if c == '}' {
        state.pos += 1;
        break;
      }

      name.push(c);
      state.pos += 1;
    }

    if let Some(c) = state.src[state.pos..].chars().next() {
      if c.is_digit(10) {
        state.pos += 1;

        let mut offset_str = c.to_string();
        while let Some(c) = state.src[state.pos..].chars().next() {
          if !c.is_digit(10) {
            break;
          }

          offset_str.push(c);
          state.pos += 1;
        }

        if let Ok(num) = offset_str.parse::<usize>() {
          offset = num;
        }
      }
    }

    let len = state.pos - start;
    state.pos = start;
    Some((Node::new(Emoji { name, offset }), len))
  }
}
