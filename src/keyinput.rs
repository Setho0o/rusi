use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crate::utils;
pub fn key_events() -> std::io::Result<()> {
  loop {
    match read()? {
      Event::Key(KeyEvent{code: KeyCode::Esc,..}) => utils::esc(),
      _ => {},
    }  
  }
  Ok(())
}
