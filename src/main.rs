mod board;
mod shapes;

use std::io::{self, stdout};
use std::time::Duration;

use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode};
use crossterm::{execute, style::Print};

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
        let _ = enable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

fn main() -> std::io::Result<()> {
    const GRAVITY_INTERVAL: u32 = 500;
    let _guard = TerminalGuard::new();

    loop {
        if event::poll(Duration::ZERO)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                _ => {}
            }
        }
    }
    Ok(())
}
