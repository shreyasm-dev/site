use content::Main;
use salvo::prelude::*;

mod content;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let acceptor = TcpListener::new("0.0.0.0:3030").bind().await;
  let router = Router::new().get(handle);
  Server::new(acceptor).serve(router).await;
}

#[handler]
async fn handle(res: &mut Response) {
  res.render(Text::Html(
    Main {
      title: "x",
      content: "hello!",
    }
    .to_string(),
  ));
}
