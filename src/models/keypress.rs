use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyPressMessage {
  pub key: u16
}