mod content;
mod markdown;
mod minify;
mod sass;
mod util;

use proc_macro::TokenStream;

#[proc_macro]
pub fn content(input: TokenStream) -> TokenStream {
  content::content(input.into()).into()
}
