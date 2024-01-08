use std::sync::Arc;

use axum::{body::Body, extract::State, http::StatusCode, middleware::Next, response::Response};
use http::Request;

pub(crate) async fn require_router_auth(
    State(secret): State<Option<Arc<str>>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if let Some(token) = secret.as_deref() {
        let auth = request
            .headers()
            .get("Router-Authorization")
            .map(|header_value| header_value.to_str().unwrap_or_default());
        match auth {
            Some(header) if header == token => (),
            _ => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap()
            }
        }
    }
    next.run(request).await
}
