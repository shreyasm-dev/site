use content::{Content, main::Main};
use salvo::prelude::*;
use util::resource_router;

mod content;
mod util;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let acceptor = TcpListener::new("0.0.0.0:3030").bind().await;

  let router = Router::new()
    .get(handle)
    .push(resource_router("style", Text::Css));

  Server::new(acceptor).serve(router).await;
}

#[handler]
async fn handle(res: &mut Response) {
  res.render(Text::Html(
    Main {
      title: None,
      content: "hello!",
    }
    .to_string(),
  ));
}
