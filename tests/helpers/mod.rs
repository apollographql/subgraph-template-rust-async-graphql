use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use axum::body::to_bytes;
use my_subgraph::app;
use serde_json::{json, Value};
use tower::ServiceExt;

pub(crate) async fn run_graphql_query(query: &str, operation: &str) -> Value {
    let app = app(None);
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({"query": query, "operationName": operation}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(status, StatusCode::OK, "{:#?}", body);

    serde_json::from_slice(&body).unwrap()
}
