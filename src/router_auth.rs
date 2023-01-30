use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use lazy_static::lazy_static;

/// A middleware that requires a `Router-Authorization` header to be present and match the value of
/// the `ROUTER_SECRET` environment variable (if set). If the environment variable is not set, this
/// does nothing.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RequireRouterAuth;

lazy_static! {
    static ref ROUTER_TOKEN: Option<String> = std::env::var("ROUTER_SECRET").ok();
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireRouterAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(token) = ROUTER_TOKEN.as_ref() {
            let auth = parts
                .headers
                .get("Router-Authorization")
                .ok_or(StatusCode::UNAUTHORIZED)?
                .to_str()
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            if auth == token {
                Ok(Self)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Ok(Self)
        }
    }
}
