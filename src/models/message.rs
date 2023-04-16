use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
  pub target: String,
  pub payload: Option<serde_json::Value>
}