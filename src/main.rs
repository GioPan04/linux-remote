use std::io::{BufReader, BufRead, Result};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::mouse_thread::mouse_handler;
mod models;
mod input;
mod mouse_thread;


fn main() -> Result<()> {
	let listener = TcpListener::bind("0.0.0.0:1234").unwrap();
	println!("Server is listening on *:1234");

	let (tx, rx) = mpsc::channel::<models::Message>();

	thread::Builder::new().name("UInput handler".into()).spawn(|| mouse_handler(rx)).expect("Failed to start the thread");	
	
	for stream in listener.incoming() {
		let socket = stream.unwrap();

		let tx = tx.clone();
		
		thread::spawn(move || {
			println!("New connection received from {:?}", socket.peer_addr().unwrap());
			handle_connection(socket, tx).unwrap();
			println!("Client closed the connection");
		});
	}

	Ok(())
}


fn handle_connection(socket: TcpStream, tx: Sender<models::Message>) -> Result<()> {
	let socket = BufReader::new(socket);

	for line in socket.lines() {
		let line = line?;

		if line.is_empty() {
			println!("Exited the connection");
			return Ok(());
		}
	
		let msg: models::Message = serde_json::from_str(&line)?;
		tx.send(msg).unwrap();
	}

	Ok(())
}