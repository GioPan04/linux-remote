use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
  pub action: u8,
  pub payload: serde_json::Value
}