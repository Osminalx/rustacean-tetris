pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

// The coordinates will work from left to right top to bottom
// cells[y][x] = ROW first, COLUMN second
pub struct Board {
    cells: [[Option<Cell>; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Option<Cell>) {
        self.cells[y][x] = cell;
    }
}
