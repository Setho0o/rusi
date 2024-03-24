use std::io;
use std::io::{Write, stdout};

use crossterm::queue;
use crossterm::style::Print;
use crossterm::cursor::MoveTo;

pub fn main_border(row: u16, column: u16) -> io::Result<()> {
  for c in 0..column {
    queue!(stdout(),
      MoveTo(0, c),Print("┃"),
      MoveTo(row, c),Print("┃")
    )?;
  }
  for r in 0..row {
    queue!(stdout(),
      MoveTo(r, 0),Print("━"),
      MoveTo(r, column),Print("━")
    )?;
  }
  queue!(stdout(),
    MoveTo(0,0),Print("┏"),
    MoveTo(row, 0), Print("┓"),
    MoveTo(0, column),Print("┗"),
    MoveTo(row,column),Print("┛")
  )?;
  stdout().flush()?;
  Ok(())
}
pub fn song_box(row: u16, column: u16) -> io::Result<()> {
  let offset_w = row / 45;
  let offset_l = column / 16; 
  let width = row - offset_w; 
  let length = column - offset_l;
  for w in offset_w..(width / 2) {
    queue!(stdout(),MoveTo(w, offset_l ), Print("━"))?;
    queue!(stdout(),MoveTo(w, length - 1), Print("━"))?;
  }
  for l in offset_l..length {
    queue!(stdout(),MoveTo(offset_w, l ), Print("┃"))?;
    queue!(stdout(),MoveTo((width / 2) - 1, l), Print("┃"))?;
  }
  queue!(stdout(),
    MoveTo(offset_w,offset_l), Print("┏"),
    MoveTo(offset_w,length - 1 ), Print("┗"),
    MoveTo((width / 2) - 1,offset_l), Print("┓"),
    MoveTo((width / 2) - 1,length - 1), Print("┛")
  )?;
  stdout().flush()?;
  Ok(())
}
