use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::redis_component;
use std::str::from_utf8;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MessageRecord {
    name: String,
    state: String,
}

/// A simple Spin Redis component.
#[redis_component]
fn on_message(message: Bytes) -> Result<()> {
    println!("Śmiga {}", from_utf8(&message)?);
    let deserialized_message: MessageRecord = serde_json::from_str(from_utf8(&message)?).unwrap();

    println!("Deserialized with {:?}", &deserialized_message);

    Ok(())
}
