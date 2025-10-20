use salvo::{catcher::Catcher, prelude::*};
use util::{PageHandler, resource_router};

use crate::util::{resource_router_str, ImageScribe};

mod util;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let acceptor = TcpListener::new("0.0.0.0:3030").bind().await;
  let router = Router::new()
    .push(resource_router_str("style", Text::Css))
    .push(resource_router("image", ImageScribe));
  Server::new(acceptor)
    .serve(Service::new(router).catcher(Catcher::new(PageHandler)))
    .await;
}
