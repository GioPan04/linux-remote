use std::fs::File;
use std::io;

use input_linux::{
  EventKind, EventTime, InputEvent, InputId, RelativeAxis, RelativeEvent, SynchronizeEvent,
  SynchronizeKind, UInputHandle, KeyEvent, KeyState, Key,
};

// https://docs.kernel.org/input/uinput.html#mouse-movements


pub fn press_key(uinput: &UInputHandle<File>, key: Key) {
	const ZERO: EventTime = EventTime::new(0, 0);

	let events = [
		*InputEvent::from(KeyEvent::new(ZERO, key, KeyState::PRESSED)).as_raw(),
		*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
		*InputEvent::from(KeyEvent::new(ZERO, key, KeyState::RELEASED)).as_raw(),
		*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
	];
	uinput.write(&events).unwrap();
}

pub fn move_cursor(uinput: &UInputHandle<File>, x: i32, y: i32) {
	const ZERO: EventTime = EventTime::new(0, 0);
	let events = [
		*InputEvent::from(RelativeEvent::new(ZERO, RelativeAxis::X, x)).as_raw(),
		*InputEvent::from(RelativeEvent::new(ZERO, RelativeAxis::Y, y)).as_raw(),
		*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
	];
	uinput.write(&events).unwrap();
}

pub fn create_mouse(uinput_file: File) -> io::Result<UInputHandle<File>> {
	let mouse = UInputHandle::new(uinput_file);

	mouse.set_evbit(EventKind::Key)?; 
	for i in 0..279 {
		mouse.set_keybit(input_linux::Key::from_code(i).unwrap())?;
	}
   
	mouse.set_evbit(EventKind::Relative)?;
	mouse.set_relbit(RelativeAxis::X)?; 
	mouse.set_relbit(RelativeAxis::Y)?;


	let input_id = InputId {
		bustype: input_linux::sys::BUS_USB,
		vendor: 0x1234,
		product: 0x5678,
		version: 0,
	};

	let device_name = b"Linux Remote Virtual Device";
	mouse.create(&input_id, device_name, 0, &[])?;

	Ok(mouse)
}