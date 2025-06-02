markup::define! {
  Style<'a>(name: &'a str) {
    link[rel = "stylesheet", href = format!("/style/{}.css", name), type = "text/css"];
  }
}
