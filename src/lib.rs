use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone)]
struct MenuItem {
    name: String,
    price: f32,
}

fn get_menu_items(): Vec<MenuItem> {

}

/// A simple Spin HTTP component.
#[http_component]
fn handle_restaurant(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")?)
}
