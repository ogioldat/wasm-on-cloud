use axum::{routing::get, Router};

async fn root() -> &'static str {
    "Siema\n"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
