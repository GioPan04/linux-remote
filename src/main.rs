use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::net::{TcpListener, TcpStream};
use std::{fs::OpenOptions, thread, time::Duration};
use std::os::unix::fs::OpenOptionsExt;

use input_linux::{UInputHandle, Key};
use nix::libc::O_NONBLOCK;
mod models;
mod input;


fn main() -> Result<()> {
	let uinput_file = OpenOptions::new()
		.read(true)
		.write(true)
		.custom_flags(O_NONBLOCK)
		.open("/dev/uinput")?;
		
	let uinput = input::create_mouse(uinput_file).unwrap();
	thread::sleep(Duration::from_secs(1));
	

	let listener = TcpListener::bind("0.0.0.0:1234").unwrap();
	println!("Server is listening on *:1234");

	loop {
		let ( socket, address ) = listener.accept()?;
		println!("New connection received from {:?}", address);

		handle_connection(socket, &uinput)?;
		println!("Client closed the connection");
	}
}


fn handle_connection(socket: TcpStream, uinput: &UInputHandle<File>) -> Result<()> {
	let socket = BufReader::new(socket);

	for line in socket.lines() {
		let line = line.unwrap();

		if line.is_empty() {
			println!("Exited the connection");
			return Ok(());
		}
	
		let msg: models::Message = serde_json::from_str(&line).unwrap();
		handle_message(msg, uinput).unwrap();

	}

	Ok(())
}

fn handle_message(msg: models::Message, uinput: &UInputHandle<File>) -> Result<()> {
	match msg.action {
		0x1 => {
			let coordinates: models::CursorMoveMessage = serde_json::from_value(msg.payload).unwrap();
			input::move_cursor(uinput, coordinates.x, coordinates.y);
		}
		0x2 => {
			let keypress: models::KeyPressMessage = serde_json::from_value(msg.payload).unwrap();
			input::press_key(uinput, Key::from_code(keypress.key)?)
		}
		_ => {}
	}

	Ok(())
}