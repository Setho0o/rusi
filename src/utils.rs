use std::io::stdout;
use std::process;
use crossterm::terminal::{disable_raw_mode, Clear, ClearType};
use crossterm::execute;
use crossterm::cursor::MoveTo;
pub fn esc() {
  disable_raw_mode().unwrap();
  execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
  process::exit(0);
}
