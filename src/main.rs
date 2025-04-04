use std::error::Error;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

async fn test() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new().route("/test", get(test));

    tracing::info!("starting server...");

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
