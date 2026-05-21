use axum::{
    routing::{get, post},
    Router,
};

mod routes;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/analyze", post(routes::analyze::analyze_resume));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Rust backend running on port 3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Backend running"
}