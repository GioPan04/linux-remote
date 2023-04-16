use mpris::Player;
use mpris::PlayerFinder;
use mpris::Event;
use std::time::Duration;
use std::{time, thread};

const RETRY_DURATION: Duration = time::Duration::from_secs(3);

pub fn player_runner() {
  let connection = PlayerFinder::new().expect("Cannot find a D-Bus connection. Killing");
  
  loop {
    let mut player = connection.find_active();
  
    while let Err(_) = player {
      thread::sleep(RETRY_DURATION);
      player = connection.find_active();
    }
    
    handle_player(player.unwrap());
    println!("Disconnected from player");
  }
}


fn handle_player(player: Player) {
  println!("Connected to player");

  let events = player.events().unwrap();

  for event in events {
    match event {
      Ok(Event::Paused) => println!("Paused"),
      Ok(Event::Playing) => println!("Playing"),
      Ok(Event::TrackChanged(track)) => println!("{:?}", track),
      Err(_) => return,
      _ => ()
    }
  }
}