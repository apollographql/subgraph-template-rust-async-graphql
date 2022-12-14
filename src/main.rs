use std::net::Ipv4Addr;

use axum::Server;
use tracing::info;

use my_subgraph::app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = app();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4001".to_string())
        .parse::<u16>()
        .unwrap();
    info!(
        "Run `rover dev --url http://localhost:{port} --name {crate_name}`",
        crate_name = env!("CARGO_PKG_NAME")
    );

    Server::bind(&(Ipv4Addr::new(0, 0, 0, 0), port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
