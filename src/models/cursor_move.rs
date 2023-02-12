use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CursorMoveMessage {
  pub x: i32,
  pub y: i32,
}