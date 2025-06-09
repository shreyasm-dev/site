use components::page::Main;
use salvo::prelude::*;

generator::content!();

pub struct ResourceHandler<T, F>(String, F)
where
  T: Scribe,
  F: Fn(String) -> T + Sync + Send;

#[async_trait]
impl<T, F> Handler for ResourceHandler<T, F>
where
  T: Scribe + 'static,
  F: Fn(String) -> T + Sync + Send + 'static,
{
  async fn handle(
    &self,
    req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
  ) {
    let path = req.param::<&str>("path").unwrap_or_default();
    if let Some((data, _)) = Content::of(&self.0, path) {
      res.render((self.1)(data));
    } else {
      res.status_code(StatusCode::NOT_FOUND);
    }
  }
}

pub fn resource_router<T: Scribe + 'static, F: Fn(String) -> T + Send + Sync + 'static>(
  kind: &str,
  scribe: F,
) -> Router {
  Router::with_path(format!("{}/{{**path}}", kind)).get(ResourceHandler(kind.to_string(), scribe))
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
            title: frontmatter.get("title").cloned(),
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
