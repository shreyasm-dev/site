use salvo::prelude::*;

use crate::content::Content;

pub struct ResourceHandler<T, F>(F)
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
    if let Some(data) = Content::of("style", path) {
      res.render((self.0)(data));
    } else {
      res.status_code(StatusCode::NOT_FOUND);
    }
  }
}

pub fn resource_router<T: Scribe + 'static, F: Fn(String) -> T + Send + Sync + 'static>(
  kind: &str,
  scribe: F,
) -> Router {
  Router::with_path(format!("{}/{{**path}}", kind)).get(ResourceHandler(scribe))
}
