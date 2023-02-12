use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
  pub x: i32,
  pub y: i32
}