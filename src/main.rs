use std::net::Ipv4Addr;

use axum::Server;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, FmtSubscriber};

use my_subgraph::app;

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
    let app = app();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4001".to_string())
        .parse::<u16>()
        .unwrap();

    info!(
        "Explore this graph at https://studio.apollographql.com/sandbox/explorer?endpoint={}",
        urlencoding::encode(&format!("http://localhost:{port}"))
    );

    Server::bind(&(Ipv4Addr::new(0, 0, 0, 0), port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
