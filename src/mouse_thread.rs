use std::sync::mpsc::Receiver;
use std::{fs::OpenOptions};
use std::os::unix::fs::OpenOptionsExt;

use input_linux::Key;
use nix::libc::O_NONBLOCK;

use crate::{models, input};


pub fn mouse_handler(rx: Receiver<models::Message>) {

  let uinput_file = OpenOptions::new()
    .read(true)
    .write(true)
    .custom_flags(O_NONBLOCK)
    .open("/dev/uinput").unwrap();

  let uinput = input::create_mouse(uinput_file).unwrap();

  for message in rx {
    match message.action {
      0x1 => {
        let coordinates: models::CursorMoveMessage = serde_json::from_value(message.payload).unwrap();
        input::move_cursor(&uinput, coordinates.x, coordinates.y);
      }
      0x2 => {
        let keypress: models::KeyPressMessage = serde_json::from_value(message.payload).unwrap();
        input::press_key(&uinput, Key::from_code(keypress.key).unwrap())
      }
      _ => {}
    }
  }
}