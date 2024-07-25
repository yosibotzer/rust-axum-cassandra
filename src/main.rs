use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let simple_collector = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter);
    
    tracing::subscriber::set_global_default(simple_collector)?;

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    let app = Router::new()
        .route("/status", get(status))
        .layer(CompressionLayer::new());
    
    info!("starting server");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn status() -> (StatusCode, String) {
    (StatusCode::OK, "OK".to_string())
}
