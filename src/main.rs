use std::fs::File;
use std::{fs::OpenOptions, thread, time::Duration};
use std::os::unix::fs::OpenOptionsExt;

use input_linux::{UInputHandle, Key};
use nix::libc::O_NONBLOCK;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream, Result};
use tokio::net::{TcpListener, TcpStream};
mod models;
mod input;

#[tokio::main]
async fn main() -> Result<()>  {
	let uinput_file = OpenOptions::new()
		.read(true)
		.write(true)
		.custom_flags(O_NONBLOCK)
		.open("/dev/uinput")?;
		
	let uinput = input::create_mouse(uinput_file).unwrap();
	thread::sleep(Duration::from_secs(1));
	

	let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();
	println!("Server is listening on *:1234");

	loop {
		let ( socket, address ) = listener.accept().await?;
		println!("New connection received from {:?}", address);

		handle_connection(socket, &uinput).await?;
	}

	// for _ in 0..50 {
	// 	move_cursor(&uinput, 5, 5);
	// 	thread::sleep(Duration::from_micros(15_000));
	// }
	
	// let keys = [Key::H, Key::E, Key::L, Key::L, Key::O, Key::Space, Key::W, Key::O, Key::R, Key::L, Key::D, Key::Enter];

	// for key in keys {
	// 	press_key(&uinput, key);
	// }


	// ;   
}


async fn handle_connection(socket: TcpStream, uinput: &UInputHandle<File>) -> Result<()> {
	let mut socket = BufStream::new(socket);
	socket.write_all(b"Hello from the server\n").await?;
	socket.flush().await?;

	let mut line = vec![];

	loop {
		line.clear();
		socket.read_until(b'\n', &mut line).await?;

		if line.is_empty() {
			println!("Exited the connection");
			return Ok(());
		}

		let msg: models::Message = serde_json::from_slice(&line).unwrap();
		handle_message(msg, uinput).unwrap();

		// socket.write_all(&line).await?;
		// socket.flush().await?;
	}
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