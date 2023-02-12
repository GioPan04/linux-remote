use std::fs::File;
use std::os::unix::fs::OpenOptionsExt;
use std::{fs::OpenOptions, io, thread, time::Duration};

use input_linux::{
  EventKind, EventTime, InputEvent, InputId, RelativeAxis, RelativeEvent, SynchronizeEvent,
  SynchronizeKind, UInputHandle, KeyEvent, KeyState, Key,
};
use nix::libc::O_NONBLOCK;

// A rust translation of the uinput example available at
// https://docs.kernel.org/input/uinput.html#mouse-movements
// Creates a virtual mouse, moves it down and to the right 250 units
// in increments of 5 units
//
// This example requires either root (bad practice, too general) or
// the running user to be a member of the uinput group to actually
// make the mouse move
fn main() -> io::Result<()> {
	let uinput_file = OpenOptions::new()
		.read(true)
		.write(true)
		.custom_flags(O_NONBLOCK)
		.open("/dev/uinput")?;


	let uinput = create_mouse(uinput_file).unwrap();


	thread::sleep(Duration::from_secs(1));
	
	for _ in 0..50 {
		move_cursor(&uinput, 5, 5);
		thread::sleep(Duration::from_micros(15_000));
	}
	
	let keys = [Key::H, Key::E, Key::L, Key::L, Key::O, Key::Space, Key::W, Key::O, Key::R, Key::L, Key::D, Key::Enter];

	for key in keys {
		press_key(&uinput, key);
	}


	uinput.dev_destroy()?;   
	Ok(())
}

fn press_key(uinput: &UInputHandle<File>, key: Key) {
	const ZERO: EventTime = EventTime::new(0, 0);

	let events = [
		*InputEvent::from(KeyEvent::new(ZERO, key, KeyState::PRESSED)).as_raw(),
		*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
		*InputEvent::from(KeyEvent::new(ZERO, key, KeyState::RELEASED)).as_raw(),
		*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
	];
	uinput.write(&events).unwrap();
}

fn move_cursor(uinput: &UInputHandle<File>, x: i32, y: i32) {
	const ZERO: EventTime = EventTime::new(0, 0);
	let events = [
		*InputEvent::from(RelativeEvent::new(ZERO, RelativeAxis::X, x)).as_raw(),
		*InputEvent::from(RelativeEvent::new(ZERO, RelativeAxis::Y, y)).as_raw(),
		*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
	];
	uinput.write(&events).unwrap();
}

fn create_mouse(uinput_file: File) -> io::Result<UInputHandle<File>> {
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