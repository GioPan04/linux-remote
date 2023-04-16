use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Track<'a> {
  pub album: Option<&'a str>,
  pub title: Option<&'a str>,
  pub artists: Option<Vec<&'a str>>
}