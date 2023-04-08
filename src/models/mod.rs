mod message;
pub use message::ClientMessage;

mod cursor_move;
pub use cursor_move::CursorMoveMessage;

mod keypress;
pub use keypress::KeyPressMessage;