use anyhow::Result;
use bytes::Bytes;
use rand::Rng;
use serde::{Deserialize, Serialize};
use spin_sdk::{redis, redis_component};
use std::{str::from_utf8, thread, time::Duration};

const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OrderRecord {
    id: String,
    name: String,
    state: String,
}

fn mark_as_ready(id: String) -> () {
    let address = std::env::var(REDIS_ADDRESS_ENV).unwrap();
    let conn = redis::Connection::open(&address).unwrap();

    let order = conn.get(&id).unwrap().unwrap();

    let mut updated_order: OrderRecord = serde_json::from_str(from_utf8(&order).unwrap()).unwrap();
    updated_order.state = "ready".to_string();

    print!("Marking order as ready {:?}", &updated_order);

    let serialized_order = serde_json::to_vec(&updated_order).unwrap();
    let _ = conn.set(&updated_order.id, &serialized_order);
}

#[redis_component]
fn prepare_order(message: Bytes) -> Result<()> {
    let deserialized_message: OrderRecord = serde_json::from_str(from_utf8(&message)?).unwrap();
    println!("Kitchen is preparing the order {:?}", deserialized_message);

    let mut rng = rand::thread_rng();
    let random_int: u64 = rng.gen_range(6..10);

    thread::sleep(Duration::new(random_int, 0));

    mark_as_ready(deserialized_message.id);

    Ok(())
}
