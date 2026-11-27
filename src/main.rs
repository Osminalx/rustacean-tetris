mod board;
mod render;
mod shapes;

use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};

use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, style::Print};

use crate::board::{Board, Cell, HEIGHT};
use crate::shapes::{Piece, can_place, cells};

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

fn tick() {
    todo!()
}

fn render(out: &mut impl Write) -> io::Result<()> {
    execute!(out, MoveTo(0, 0), Clear(ClearType::All),)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    const GRAVITY_INTERVAL: Duration = Duration::from_millis(500);

    let _guard = TerminalGuard::new()?;
    let mut last_frame = Instant::now();
    let mut acc = Duration::ZERO;
    let mut out = stdout();
    let mut board = Board::new();
    let mut piece = Piece {
        kind: Cell::L,
        x: 5,
        y: 2,
    };

    // let (origin_x, origin_y) = (5, 2);
    // let piece = cells(Cell::L);
    // for &(dx, dy) in &piece {
    //     let Ok(x) = usize::try_from(origin_x + dx) else {
    //         continue;
    //     };
    //     let Ok(y) = usize::try_from(origin_y + dy) else {
    //         continue;
    //     };
    //
    //     board.set(x, y, Some(Cell::L));
    // }

    loop {
        let now = Instant::now();
        acc += now - last_frame;
        last_frame = now;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                _ => {}
            }
        }

        while acc >= GRAVITY_INTERVAL {
            if can_place(&board, &piece, piece.x, piece.y + 1) {
                piece.y += 1;
            }
            acc -= GRAVITY_INTERVAL;
        }

        render::draw(&board, &piece, &mut out)?
    }
    Ok(())
}
