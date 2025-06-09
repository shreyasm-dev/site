use std::collections::HashMap;

use crate::util::Output;

pub fn verbatim(input: &str) -> Output {
  Output {
    metadata: HashMap::new(),
    content: input.to_string(),
  }
}
