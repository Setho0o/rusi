use std::io::stdout;

use tokio::task;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use crossterm::cursor::Hide;

use audio::player::player;
use ui::*;
use keyinput::key_events;
mod audio;
mod ui;
mod utils;
mod keyinput;
#[tokio::main] 
async fn main() {
  let _music_future = task::spawn(music());
  let _key_future = task::spawn(keys());
  let (row, column) = match size() {
     Ok(size) => size,
     Err(_) => panic!("could not get size"),
  };
  
  enable_raw_mode().unwrap();
  execute!(stdout(), Clear(ClearType::All),Hide).unwrap();
  let _ = main_border(row, column);
  let _ = song_box(row, column);
}
async fn music() {
  player("audio/songs/Doomsday.flac");
}
async fn keys() {
  key_events();
}
