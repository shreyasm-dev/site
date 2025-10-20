use components::page::Main;
use salvo::{
  http::{HeaderValue, header::CONTENT_TYPE},
  prelude::*,
};

generator::content!();

pub struct ImageScribe(pub Vec<u8>);

impl Scribe for ImageScribe {
  fn render(self, res: &mut Response) {
    res
      .headers_mut()
      .insert(CONTENT_TYPE, HeaderValue::from_str("image/jpeg").unwrap());
    res.write_body(self.0).unwrap();
  }
}

pub struct ResourceHandler<T, F>(String, F)
where
  T: Scribe,
  F: Fn(Vec<u8>) -> T + Sync + Send;

#[async_trait]
impl<T, F> Handler for ResourceHandler<T, F>
where
  T: Scribe + 'static,
  F: Fn(Vec<u8>) -> T + Sync + Send + 'static,
{
  async fn handle(
    &self,
    req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
  ) {
    let path = req.param::<&str>("path").unwrap_or_default();
    if let Some((data, _)) = Content::of_raw(&self.0, path) {
      res.render((self.1)(data));
    } else {
      res.status_code(StatusCode::NOT_FOUND);
    }
  }
}

pub fn resource_router<T: Scribe + 'static, F: Fn(Vec<u8>) -> T + Send + Sync + 'static>(
  kind: &str,
  scribe: F,
) -> Router {
  Router::with_path(format!("{}/{{**path}}", kind)).get(ResourceHandler(kind.to_string(), scribe))
}

pub fn resource_router_str<T: Scribe + 'static, F: Fn(String) -> T + Send + Sync + 'static>(
  kind: &str,
  scribe: F,
) -> Router {
  resource_router(kind, move |content| {
    scribe(String::from_utf8(content).unwrap())
  })
}

pub struct PageHandler;

#[async_trait]
impl Handler for PageHandler {
  async fn handle(
    &self,
    req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
  ) {
    for suffix in [".md", "/index.md"] {
      if let Some((content, frontmatter)) =
        Content::of("page", &format!("{}{}", req.uri().path(), suffix))
      {
        res.render(Text::Html(
          Main {
            title: frontmatter.title,
            content: &content,
          }
          .to_string(),
        ));
        res.status_code(StatusCode::OK);
        break;
      }
    }
  }
}
