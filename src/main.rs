mod board;
mod shapes;

use crossterm::cursor::MoveTo;
use crossterm::event::poll;
use crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode};

fn render() {
    println!("it's rendering!");
}

fn main() {
    const GRAVITY_INTERVAL: u32 = 500;
    loop {}
}
