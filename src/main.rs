use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber, layer::SubscriberExt, util::SubscriberInitExt};

use my_subgraph::{app, graceful_shutdown};

#[tokio::main]
async fn main() {
    FmtSubscriber::new()
        .with(
            EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env()
                .expect("Could not set up tracing subscriber"),
        )
        .init();
    let app = app(std::env::var("ROUTER_SECRET").ok());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4001".to_string())
        .parse::<u16>()
        .unwrap();

    info!(
        "Explore this graph at https://studio.apollographql.com/sandbox/explorer?endpoint={}",
        urlencoding::encode(&format!("http://localhost:{port}"))
    );

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Could not bind to port");
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}
