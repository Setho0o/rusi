use std::io::stdout;
use std::process;
use std::fs;
use std::path::Path;
use std::vec::Vec;
use crossterm::terminal::{disable_raw_mode, Clear, ClearType};
use crossterm::execute;
use crossterm::cursor::MoveTo;
pub fn esc() {
  disable_raw_mode().unwrap();
  execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
  process::exit(0);
}
pub fn read_songs() -> Vec<String> {
  let mut songs = Vec::new();
  let mut index = 0; 
  for entry in fs::read_dir("./audio/songs").expect("could not read song dir") {
    let dir = entry.expect("no entrys");
    songs.push(dir.path().to_str().expect("PathBuf to str conversion failed").to_string());
    let mut song_clean = songs[index].split_off(14);
    songs[index] = song_clean;
    index += 1;
  }
  return songs  
} 
