use std::{thread, net::{TcpListener, TcpStream}, sync::mpsc::Sender};
use std::io::{BufRead, BufReader, Result};

use crate::models;

pub fn setup_connection(listener: TcpListener, tx: Sender<models::ClientMessage>) -> Result<()> {
  for stream in listener.incoming() {
		let socket = stream.unwrap();

		let tx = tx.clone();
		
		thread::Builder::new()
			.name(format!("Connection {:?}", socket.peer_addr()))
			.spawn(move || {
				println!("New connection received from {:?}", socket.peer_addr().unwrap());
				let _ = handle_connection(&socket, tx);
				println!("Client closed the connection");
			})?;
	}

  Ok(())
}


fn handle_connection(socket: &TcpStream, tx: Sender<models::ClientMessage>) -> Result<()> {
	let socket = BufReader::new(socket);

	for line in socket.lines() {
		let line = line?;

		if line.is_empty() {
			println!("Exited the connection");
			return Ok(());
		}
	
		match serde_json::from_str::<models::ClientMessage>(&line) {
			Ok(msg) => tx.send(msg).unwrap(),
			Err(err) => eprintln!("Couldn't decode message: {:?}", err)
		}

	}

	Ok(())
}