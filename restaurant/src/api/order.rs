use std::str::from_utf8;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{
        responses::{internal_server_error, not_found},
        IntoResponse, Params, Request, Response,
    },
    redis::{self},
};

const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";
const REDIS_CHANNEL_ENV: &str = "REDIS_CHANNEL";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OrderRecord {
    name: String,
    state: String,
}

pub fn place_order(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let channel = std::env::var(REDIS_CHANNEL_ENV)?;
    let conn = redis::Connection::open(&address)?;

    let order_name = params.get("name").expect("Invalid param");

    let payload = OrderRecord {
        state: "placed".to_string(),
        name: order_name.to_string(),
    };
    let serialized_payload = serde_json::to_vec(&payload).unwrap();

    print!("Placing an order for {}", order_name);

    conn.set(&payload.name, &serialized_payload)?;

    match conn.publish(&channel, &serialized_payload) {
        Ok(()) => Ok(Response::new(200, ())),
        Err(_e) => Ok(internal_server_error()),
    }
}

pub fn check_order(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let conn = redis::Connection::open(&address)?;

    let order_name = params.get("name").expect("Invalid param");

    print!("Checking order for {}", order_name);

    match conn.get(order_name) {
        Ok(Some(order)) => Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(from_utf8(&order)?)
            .build()),
        Ok(None) => Ok(not_found()),
        Err(_err) => Ok(internal_server_error()),
    }
}
