use mpris::Player;
use mpris::PlayerFinder;
use mpris::Event;
use tokio::sync::broadcast::Sender;
use std::time::Duration;
use std::{time, thread};

use crate::models::ClientMessage;
use crate::models::Track;

const RETRY_DURATION: Duration = time::Duration::from_secs(3);

pub fn player_runner(tx: Sender<ClientMessage>) {
  let connection = PlayerFinder::new().expect("Cannot find a D-Bus connection. Killing");
  
  loop {
    let mut player = connection.find_active();
  
    while let Err(_) = player {
      thread::sleep(RETRY_DURATION);
      player = connection.find_active();
    }
    
    handle_player(player.unwrap(), tx.clone());
    println!("Disconnected from player");
  }
}


fn handle_player(player: Player, tx: Sender<ClientMessage>) {
  println!("Connected to player");

  let events = player.events().unwrap();

  for event in events {
    let target: ClientMessage = match event {
      Ok(Event::Paused) => ClientMessage { target: "player:paused".into(), payload: None },
      Ok(Event::Playing) => ClientMessage { target: "player:playing".into(), payload: None },
      Ok(Event::TrackChanged(track)) => ClientMessage {
        target: "player:track_changed".into(),
        payload: Some(serde_json::json!(Track {
          title: track.title(),
          album: track.album_name(),
          artists: track.artists() 
        }))
      },
      Err(_) => return,
      _ => return,
    };

    tx.send(target).unwrap();
  }
}