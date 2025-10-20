markup::define! {
  Style<'a>(name: &'a str) {
    link[rel = "stylesheet", href = format!("/css/{}.css", name), type = "text/css"];
  }
}
