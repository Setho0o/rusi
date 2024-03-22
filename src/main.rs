use std::io::stdout;

use tokio::task;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};

use audio::player::player;
use ui::main_border;
mod audio;
mod ui;
#[tokio::main] 
async fn main() {
  let concurent_future = task::spawn(music());

  let (row, column) = match size() {
     Ok(size) => size,
     Err(_) => panic!("could not get size"),
  };
  enable_raw_mode().unwrap();
  execute!(stdout(), Clear(ClearType::All)).unwrap();
  main_border(row, column);
  disable_raw_mode();
}
async fn music() {
  player("audio/songs/Doomsday.flac");
}
