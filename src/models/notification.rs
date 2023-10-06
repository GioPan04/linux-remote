use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotificationMessage {
  pub unique_id: String,
  pub title: String,
  pub text: String,
}