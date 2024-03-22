use std::io::{Write, stdout};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType, size};
pub fn main_border(row: u16, column: u16){
  for c in 0..column {
    queue!(stdout(),
      MoveTo(0, c),Print("┃"),
      MoveTo(row, c),Print("┃")
    );
  }
  for r in 0..row {
    queue!(stdout(),
      MoveTo(r, 0),Print("━"),
      MoveTo(r, column),Print("━")
    );
  }
  queue!(stdout(),
    MoveTo(0,0),Print("┏"),
    MoveTo(row, 0), Print("┓"),
    MoveTo(0, column),Print("┗"),
    MoveTo(row,column),Print("┛")
  );
  stdout().flush();
} 
