use std::fs::File;
use std::io;
use std::sync::mpsc::Receiver;
use std::{fs::OpenOptions};
use std::os::unix::fs::OpenOptionsExt;

use input_linux::{Key, UInputHandle};
use nix::libc::O_NONBLOCK;

use crate::{models, input};


pub fn mouse_handler(rx: Receiver<models::Message>) -> io::Result<()> {

  let uinput_file = OpenOptions::new()
    .read(true)
    .write(true)
    .custom_flags(O_NONBLOCK)
    .open("/dev/uinput")?;

  let uinput = input::create_mouse(uinput_file).expect("Cannot create virtual device");

  for message in rx {
    if let Err(err) = decode_message(message, &uinput) {
      eprintln!("Couldn't decode message: {:?}", err);
    }
  }

  Ok(())
}

fn decode_message(message: models::Message, uinput: &UInputHandle<File>) -> io::Result<()> {
  match message.action {
    0x1 => {
      let coordinates: models::CursorMoveMessage = serde_json::from_value(message.payload)?;
      input::move_cursor(&uinput, coordinates.x, coordinates.y);
    }
    0x2 => {
      let keypress: models::KeyPressMessage = serde_json::from_value(message.payload)?;
      input::press_key(&uinput, Key::from_code(keypress.key)?)
    }
    _ => {}
  }

  Ok(())
}