use std::io::Result;
use serde_json::json;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;

use crate::models::{self, ClientMessage};

pub async fn setup_connection(listener: TcpListener, tx: Sender<ClientMessage>) -> Result<()> {
  loop {
		let (socket, _) = listener.accept().await?;

		let tx = tx.clone();
		let rx = tx.subscribe();
		
		tokio::spawn(async move {
			println!("New connection received from {:?}", socket.peer_addr().unwrap());
			let _ = handle_connection(socket, tx, rx).await;
			println!("Client closed the connection");
		});
	}
}


async fn handle_connection(mut socket: TcpStream, tx: Sender<models::ClientMessage>, mut rx: Receiver<ClientMessage>) -> Result<()> {
	let (reader, mut writer) = socket.split();
	let mut reader = BufReader::new(reader);
	let mut line = String::new();

	loop {
		tokio::select! {
			result = reader.read_line(&mut line) => {
				if result.unwrap() == 0 {
					break;
				}
				
				match serde_json::from_str::<models::ClientMessage>(&line) {
					Ok(msg) => { tx.send(msg).unwrap(); },
					Err(err) => eprintln!("Couldn't decode message: {:?}", err)
				}

				line.clear();
			}
			result = rx.recv() => {
				let mut msg = json!(result.unwrap()).to_string();
				msg.push('\n');
				writer.write_all(msg.as_bytes()).await.unwrap();
			}
		}

	}

	Ok(())
}