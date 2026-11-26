use std::io::{self, Write};

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;

use crate::{
    board::{Board, HEIGHT, WIDTH},
    shapes::{Piece, cells},
};

pub fn draw(board: &Board, piece: &Piece, out: &mut impl Write) -> io::Result<()> {
    let mut frame = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board.cell(x, y).is_some() {
                frame.push_str("██")
            } else {
                let mut drawn = false;
                for (dx, dy) in cells(piece.kind) {
                    if piece.x + dx == x as i32 && piece.y + dy == y as i32 {
                        frame.push_str("██");
                        drawn = true;
                        break;
                    }
                }
                if !drawn {
                    frame.push_str("· ");
                }
            }
        }
        frame.push_str("\r\n");
    }

    execute!(out, MoveTo(0, 0), Print(frame))
}
