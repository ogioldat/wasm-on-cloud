use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Params, Request, Response};

pub fn get_menu(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    println!("Serving menu");
    let menu_items = get_menu_items();
    let menu_items_serialized = serde_json::to_string(&menu_items)?;
    Ok(Response::new(200, Some(menu_items_serialized)))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MenuItem {
    name: String,
    price: f32,
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

pub fn place_order(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(200, Some("test")))
}

// // /hello/:planet
// pub async fn hello_planet(_req: Request, params: Params) -> Result<impl IntoResponse> {
//     let planet = params.get("planet").expect("PLANET");
//     Ok(Response::new(200, planet.to_string()))
// }

// // /*
// pub async fn echo_wildcard(_req: Request, params: Params) -> Result<impl IntoResponse> {
//     let capture = params.wildcard().unwrap_or_default();
//     Ok(Response::new(200, capture.to_string()))
// }
