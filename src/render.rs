use std::io::{self, Write};

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;

use crate::board::{Board, HEIGHT, WIDTH};

pub fn draw(board: &Board, out: &mut impl Write) -> io::Result<()> {
    let mut frame = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match board.cell(x, y) {
                Some(_) => frame.push_str("██"),
                None => frame.push_str("· "),
            }
        }
        frame.push_str("\r\n");
    }

    execute!(out, MoveTo(0, 0), Print(frame))
}
