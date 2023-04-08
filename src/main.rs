use std::io::Result;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

mod models;
mod input;
mod mouse_thread;
mod player_thread;
mod connection;


fn main() -> Result<()> {
	let listener = TcpListener::bind("0.0.0.0:1234").unwrap();
	println!("Server is listening on *:1234");

	let (tx, rx) = mpsc::channel::<models::ClientMessage>();

	thread::Builder::new()
		.name("UInput handler".into())
		.spawn(move || mouse_thread::mouse_handler(rx))?;	

	connection::setup_connection(listener, tx)?;

	Ok(())
}