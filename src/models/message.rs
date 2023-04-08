use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
  pub target: String,
  pub payload: serde_json::Value
}