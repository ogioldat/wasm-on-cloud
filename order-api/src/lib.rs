use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;
use spin_sdk::redis;

const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";

/// A simple Spin HTTP component.
#[http_component]
async fn handle_kitchen_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Request started");

    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let conn = redis::Connection::open(&address)?;

    conn.set("test", &"to powinno dzialac".into());

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello from order")?)
}

