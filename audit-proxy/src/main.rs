use axum::{
    routing::{get, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    println!("--- Audit Proxy ---");
    
    // Set up tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        // Placeholder for S3 proxy routes
        .route("/*key", put(intercept_s3_put))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "SailOverRustFS Audit Proxy"
}

async fn health() -> &'static str {
    "OK"
}

async fn intercept_s3_put() -> &'static str {
    println!("Intercepted PutObject request");
    "Proxy Ack"
}
