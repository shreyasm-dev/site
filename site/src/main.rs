use salvo::{catcher::Catcher, prelude::*};
use util::{PageHandler, resource_router};

mod util;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let acceptor = TcpListener::new("0.0.0.0:3030").bind().await;
  let router = Router::new().push(resource_router("style", Text::Css));
  Server::new(acceptor)
    .serve(Service::new(router).catcher(Catcher::new(PageHandler)))
    .await;
}
