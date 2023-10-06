use std::io;
use notify_rust::Notification;
use tokio::sync::broadcast::Receiver;

use crate::models;


pub async fn notification_handler(mut rx: Receiver<models::ClientMessage>) -> io::Result<()> {  
  loop {
    let message = rx.recv().await.unwrap();
    if message.target.as_str() != "notification:show" {
      continue;
    }

    match serde_json::from_value::<models::NotificationMessage>(message.payload.unwrap()) {
      Ok(value) => {
        Notification::new()
          .summary(&value.title)
          .body(&value.text)
          .show()
          .unwrap();
      },
      Err(e) => {
        eprintln!("Couldn't decode message: {:?}", e);
      } 
    }
  }
}