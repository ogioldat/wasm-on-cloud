use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{
        responses::{internal_server_error, not_found},
        IntoResponse, Params, Request, Response,
    },
    redis::{self},
};
use std::str::from_utf8;
use uuid::Uuid;

const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";
const REDIS_CHANNEL_ENV: &str = "REDIS_CHANNEL";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OrderRecord {
    id: String,
    name: String,
    state: String,
}

pub fn place_order(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let channel = std::env::var(REDIS_CHANNEL_ENV)?;
    let conn = redis::Connection::open(&address)?;

    let order_name = params.get("name").expect("Invalid param");

    let order = OrderRecord {
        id: Uuid::new_v4().to_string(),
        state: "placed".to_string(),
        name: order_name.to_string(),
    };
    let serialized_payload = serde_json::to_vec(&order).unwrap();

    print!("Placing an order for {}", order_name);

    conn.set(&order.id, &serialized_payload)?;

    match conn.publish(&channel, &serialized_payload) {
        Ok(()) => Ok(Response::new(200, serde_json::to_string(&order)?)),
        Err(_e) => Ok(internal_server_error()),
    }
}

pub fn check_order(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let conn = redis::Connection::open(&address)?;

    let order_id = params.get("id").expect("Invalid param");

    print!("Checking order for {}", order_id);

    match conn.get(order_id) {
        Ok(Some(order)) => Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(from_utf8(&order)?)
            .build()),
        Ok(None) => Ok(not_found()),
        Err(_err) => Ok(internal_server_error()),
    }
}
