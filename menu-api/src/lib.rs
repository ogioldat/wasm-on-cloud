use http::{Request, Response};
use serde::{Deserialize, Serialize};
use spin_sdk::{http::IntoResponse, http_component};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MenuItem {
    name: String,
    price: f32,
}

/// A simple Spin HTTP component.
#[http_component]
async fn handle_hello_rust(_req: Request<()>) -> anyhow::Result<impl IntoResponse> {
    let menu_items = get_menu_items();
    let menu_items_serialized = serde_json::to_string(&menu_items)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(menu_items_serialized))?)
}

fn get_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "PIZZA".to_string(),
            price: 25.0,
        },
        MenuItem {
            name: "CARBONARA".to_string(),
            price: 18.0,
        },
        MenuItem {
            name: "CALZONE".to_string(),
            price: 20.0,
        },
        MenuItem {
            name: "GNOCCHI".to_string(),
            price: 17.0,
        },
    ]
}
