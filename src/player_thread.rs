use mpris::PlayerFinder;
use mpris::Event;

pub fn player_runner() {
  let finder = PlayerFinder::new().unwrap();

  let player = finder.find_active().unwrap();

  let events = player.events().unwrap();

  for event in events {
    match event.unwrap() {
      Event::Paused => println!("Paused"),
      Event::Playing => println!("Playing"),
      Event::TrackChanged(track) => println!("{:?}", track),
      _ => ()
    }
  }
}