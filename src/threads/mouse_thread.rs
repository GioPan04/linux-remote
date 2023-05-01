use std::fs::File;
use std::io;
use std::{fs::OpenOptions};
use std::os::unix::fs::OpenOptionsExt;

use input_linux::{Key, UInputHandle};
use nix::libc::O_NONBLOCK;
use tokio::sync::broadcast::Receiver;

use crate::{models, input};


pub async fn mouse_handler(mut rx: Receiver<models::ClientMessage>) -> io::Result<()> {
  let uinput_file = OpenOptions::new()
    .read(true)
    .write(true)
    .custom_flags(O_NONBLOCK)
    .open("/dev/uinput")?;

  let uinput = input::create_mouse(uinput_file).expect("Cannot create virtual device");
  
  loop {
    let message = rx.recv().await.unwrap();
    if let Err(err) = decode_message(message, &uinput) {
      eprintln!("Couldn't decode message: {:?}", err);
    }
  }
}

fn decode_message(message: models::ClientMessage, uinput: &UInputHandle<File>) -> io::Result<()> {
  match message.target.as_str() {
    "uinput:cursor_move" => {
      let coordinates: models::CursorMoveMessage = serde_json::from_value(message.payload.unwrap())?;
      input::move_cursor(&uinput, coordinates.x, coordinates.y);
    }
    "uinput:key_press" => {
      let key: u16 = serde_json::from_value(message.payload.unwrap())?;
      input::press_key(&uinput, Key::from_code(key)?);
    }
    _ => {}
  }

  Ok(())
}