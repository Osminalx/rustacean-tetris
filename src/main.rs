mod board;
mod shapes;

use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};

use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
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
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

fn tick(counter: &mut i32) {
    *counter += 1
}

fn render(out: &mut impl Write, counter: i32) -> io::Result<()> {
    execute!(
        out,
        MoveTo(0, 0),
        Clear(ClearType::All),
        Print(format!("counter {counter}")),
    )?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    const GRAVITY_INTERVAL: Duration = Duration::from_millis(500);

    let _guard = TerminalGuard::new()?;
    let mut last_frame = Instant::now();
    let mut acc = Duration::ZERO;
    let mut out = stdout();
    let mut counter = 0;

    loop {
        let now = Instant::now();
        acc += now - last_frame;
        last_frame = now;

        if event::poll(Duration::ZERO)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                _ => {}
            }
        }

        while acc >= GRAVITY_INTERVAL {
            tick(&mut counter); // game logic here
            acc -= GRAVITY_INTERVAL;
        }

        render(&mut out, counter)?
    }
    Ok(())
}
