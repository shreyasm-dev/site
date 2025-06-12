use crate::util::{Metadata, Output};

pub fn verbatim(input: &[u8]) -> Output {
  Output {
    metadata: Metadata::default(),
    content: input.to_vec(),
  }
}
