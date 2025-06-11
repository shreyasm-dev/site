use crate::util::{Metadata, Output};

pub fn verbatim(input: &str) -> Output {
  Output {
    metadata: Metadata::default(),
    content: input.to_string(),
  }
}
