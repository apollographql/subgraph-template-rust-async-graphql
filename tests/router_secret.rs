use axum::body::{to_bytes, Body, Bytes};
use http::{Request, StatusCode};
use my_subgraph::app;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn run_graphql_query(
    query: &str,
    app_secret: Option<String>,
    header_secret: Option<String>,
) -> (StatusCode, Bytes) {
    let app = app(app_secret);
    let mut request = Request::builder()
        .method(http::Method::POST)
        .uri("/")
        .header(http::header::CONTENT_TYPE, "application/json");
    if let Some(header_secret) = header_secret {
        request = request.header("Router-Authorization", header_secret);
    }
    let response = app
        .oneshot(
            request
                .body(Body::from(json!({"query": query}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, body)
}

#[tokio::test]
async fn test_missing_secret() {
    let (status, body) = run_graphql_query(
        r#"
        query {
            _service {
                sdl
            }
        }
        "#,
        Some("secret".to_string()),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(body.is_empty())
}

#[tokio::test]
async fn test_incorrect_secret() {
    let (status, body) = run_graphql_query(
        r#"
        query {
            _service {
                sdl
            }
        }
        "#,
        Some("secret".to_string()),
        Some("secret2".to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(body.is_empty())
}

#[tokio::test]
async fn test_authorized() {
    let (status, body) = run_graphql_query(
        r#"
        query {
            _service {
                sdl
            }
        }
        "#,
        Some("secret".to_string()),
        Some("secret".to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let json = serde_json::from_slice::<Value>(&body).expect("invalid json");
    assert!(json.get("data").is_some());
    assert!(json.get("errors").is_none());
}
