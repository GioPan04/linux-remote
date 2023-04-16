use std::io::Result;
use std::thread;

use tokio::net::TcpListener;
use tokio::sync::broadcast;

mod models;
mod input;
mod mouse_thread;
mod player_thread;
mod connection;


#[tokio::main]
async fn main() -> Result<()> {
	let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();
	println!("Server is listening on *:1234");

	let (tx, _) = broadcast::channel::<models::ClientMessage>(16);

	let uinput_rx = tx.clone().subscribe();
	tokio::spawn(async move { mouse_thread::mouse_handler(uinput_rx).await });

	let player_tx = tx.clone();
	thread::Builder::new()
		.name("Player handler".into())
		.spawn(move || player_thread::player_runner(player_tx))?;

	connection::setup_connection(listener, tx).await?;

	Ok(())
}